/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IsoCurrencyCode : An ISO-4217 currency code.

/// An ISO-4217 currency code.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IsoCurrencyCode {
    #[serde(rename = "USD")]
    Usd,

}

impl ToString for IsoCurrencyCode {
    fn to_string(&self) -> String {
        match self {
            Self::Usd => String::from("USD"),
        }
    }
}

impl Default for IsoCurrencyCode {
    fn default() -> IsoCurrencyCode {
        Self::Usd
    }
}




