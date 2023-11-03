/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WalletIsoCurrencyCode : An ISO-4217 currency code, used with e-wallets and transactions.

/// An ISO-4217 currency code, used with e-wallets and transactions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WalletIsoCurrencyCode {
    #[serde(rename = "GBP")]
    Gbp,
    #[serde(rename = "EUR")]
    Eur,

}

impl ToString for WalletIsoCurrencyCode {
    fn to_string(&self) -> String {
        match self {
            Self::Gbp => String::from("GBP"),
            Self::Eur => String::from("EUR"),
        }
    }
}

impl Default for WalletIsoCurrencyCode {
    fn default() -> WalletIsoCurrencyCode {
        Self::Gbp
    }
}




