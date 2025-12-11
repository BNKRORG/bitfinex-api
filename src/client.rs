//! Bitfinex client

use std::time::{Duration, SystemTime};

use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Method, Response};
use serde::de::DeserializeOwned;
use url::Url;

use crate::auth::{self, BitfinexAuth};
use crate::constant::{API_ROOT_URL, API_SIGNATURE_PATH, USER_AGENT_NAME};
use crate::error::Error;
use crate::response::Wallet;

enum Api {
    Wallets,
}

impl Api {
    fn url_path(&self) -> &str {
        match self {
            Self::Wallets => "/v2/auth/r/wallets",
        }
    }

    fn http_method(&self) -> Method {
        match self {
            Self::Wallets => Method::POST,
        }
    }
}

/// Bitfinex client
#[derive(Debug, Clone)]
pub struct BitfinexClient {
    /// Root URL for the API.
    root_url: Url,
    /// HTTP client.
    client: Client,
    /// Authentication
    auth: BitfinexAuth,
}

impl BitfinexClient {
    /// Construct a new client.
    pub fn new(auth: BitfinexAuth) -> Result<Self, Error> {
        Ok(Self {
            root_url: Url::parse(API_ROOT_URL)?,
            client: Client::builder()
                .user_agent(USER_AGENT_NAME)
                .timeout(Duration::from_secs(25))
                .build()?,
            auth,
        })
    }

    fn build_headers(&self, api: &Api, payload: Option<String>) -> Result<HeaderMap, Error> {
        let nonce: u64 = generate_nonce();
        let payload: String = payload.unwrap_or_default();

        let signature_path: String =
            format!("{API_SIGNATURE_PATH}{}{nonce}{payload}", api.url_path());

        let mut headers = HeaderMap::with_capacity(5);

        // Set content type and accept
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        // Add nonce
        headers.insert(
            HeaderName::from_static("bfx-nonce"),
            HeaderValue::from_str(nonce.to_string().as_str())?,
        );

        match &self.auth {
            BitfinexAuth::ApiKeys {
                api_key,
                api_secret,
            } => {
                // Sign payload
                let signature: String = auth::sign_payload(api_secret, signature_path)?;

                headers.insert(
                    HeaderName::from_static("bfx-apikey"),
                    HeaderValue::from_str(api_key)?,
                );
                headers.insert(
                    HeaderName::from_static("bfx-signature"),
                    HeaderValue::from_str(signature.as_str())?,
                );
            }
        }

        Ok(headers)
    }

    async fn call_api<T>(&self, api: Api) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let url: Url = self.root_url.join(api.url_path())?;
        let method: Method = api.http_method();

        // Build headers
        let headers: HeaderMap = self.build_headers(&api, None)?;

        // Send request
        let response: Response = self
            .client
            .request(method, url)
            .headers(headers)
            .send()
            .await?;

        // Propagate error if any
        let response: Response = response.error_for_status()?;

        // Deserialize response
        Ok(response.json().await?)
    }

    /// Get wallets
    ///
    /// <https://docs.bitfinex.com/reference/rest-auth-wallets>
    #[inline]
    pub async fn wallets(&self) -> Result<Vec<Wallet>, Error> {
        self.call_api(Api::Wallets).await
    }
}

fn generate_nonce() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
