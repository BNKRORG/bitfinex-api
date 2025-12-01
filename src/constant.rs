pub(crate) const API_ROOT_URL: &str = "https://api.bitfinex.com/v2";
pub(crate) const API_SIGNATURE_PATH: &str = "/api/v2/auth/r/";

pub(crate) const USER_AGENT_NAME: &str = concat!("strike-api/", env!("CARGO_PKG_VERSION"));
