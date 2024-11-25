/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AuthUpdateTypes : The possible types of auth data that may have changed.

/// The possible types of auth data that may have changed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthUpdateTypes {
    #[serde(rename = "ACCOUNT_NUMBER")]
    AccountNumber,
    #[serde(rename = "ROUTING_NUMBER")]
    RoutingNumber,

}

impl ToString for AuthUpdateTypes {
    fn to_string(&self) -> String {
        match self {
            Self::AccountNumber => String::from("ACCOUNT_NUMBER"),
            Self::RoutingNumber => String::from("ROUTING_NUMBER"),
        }
    }
}

impl Default for AuthUpdateTypes {
    fn default() -> AuthUpdateTypes {
        Self::AccountNumber
    }
}




