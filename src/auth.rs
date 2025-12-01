//! Bitfinex authentication

use std::fmt;

/// Bitfinex authentication
#[derive(Clone)]
pub enum BitfinexAuth {
    /// API Keys
    ApiKeys {
        /// API Key
        api_key: String,
        /// Secret Key
        api_secret: String,
    },
}

impl fmt::Debug for BitfinexAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BitfinexAuth").finish()
    }
}

impl BitfinexAuth {
    /// Construct API keys credential
    pub fn api_keys<K, S>(api_key: K, api_secret: S) -> Self
    where
        K: Into<String>,
        S: Into<String>,
    {
        Self::ApiKeys {
            api_key: api_key.into(),
            api_secret: api_secret.into(),
        }
    }
}
