//! Bitfinex responses

use serde::Deserialize;
use serde_json::{Map, Value};

/// Bitfinex wallet
///
/// <https://docs.bitfinex.com/reference/rest-auth-wallets>
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(from = "WalletArray")]
pub struct Wallet {
    /// Wallet type
    pub r#type: String,
    /// Currency
    pub currency: String,
    /// Balance
    pub balance: f64,
    /// Unsettled interest
    pub unsettled_interest: f64,
    /// Wallet balance available for orders/withdrawal/transfer
    pub available_balance: f64,
    /// Description of the last ledger entry
    pub last_change: String,
    /// Optional object with details
    pub last_change_metadata: Map<String, Value>,
}

#[derive(Deserialize)]
struct WalletArray(
    String,             // type
    String,             // currency
    f64,                // balance
    f64,                // unsettled_interest
    f64,                // available_balance
    String,             // last_change
    Map<String, Value>, // trade_details
);

impl From<WalletArray> for Wallet {
    fn from(arr: WalletArray) -> Self {
        Wallet {
            r#type: arr.0,
            currency: arr.1,
            balance: arr.2,
            unsettled_interest: arr.3,
            available_balance: arr.4,
            last_change: arr.5,
            last_change_metadata: arr.6,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_wallet_deserialization() {
        let json = r#"["exchange","UST",19788.6529257,0,19788.6529257,"Exchange 2.0 UST for USD @ 11.696",{
  		"reason": "TRADE",
  		"order_id": 1189740779,
  		"order_id_oppo": 1189785673,
  		"trade_price": "11.696",
  		"trade_amount": "-2.0",
  		"order_cid": 1598516362757,
  		"order_gid": 1598516362629
  	}
  ]"#;

        let wallet: Wallet = serde_json::from_str(json).unwrap();

        let mut expected_metadata = Map::new();
        expected_metadata.insert("reason".to_string(), json!("TRADE"));
        expected_metadata.insert("order_id".to_string(), json!(1189740779));
        expected_metadata.insert("order_id_oppo".to_string(), json!(1189785673));
        expected_metadata.insert("trade_price".to_string(), json!("11.696"));
        expected_metadata.insert("trade_amount".to_string(), json!("-2.0"));
        expected_metadata.insert("order_cid".to_string(), json!(1598516362757u64));
        expected_metadata.insert("order_gid".to_string(), json!(1598516362629u64));

        assert_eq!(
            wallet,
            Wallet {
                r#type: String::from("exchange"),
                currency: String::from("UST"),
                balance: 19788.6529257,
                unsettled_interest: 0.0,
                available_balance: 19788.6529257,
                last_change: String::from("Exchange 2.0 UST for USD @ 11.696"),
                last_change_metadata: expected_metadata
            }
        );
    }
}
