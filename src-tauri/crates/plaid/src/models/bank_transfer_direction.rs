/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BankTransferDirection : Indicates the direction of the transfer: `outbound` for API-initiated transfers, or `inbound` for payments received by the FBO account.

/// Indicates the direction of the transfer: `outbound` for API-initiated transfers, or `inbound` for payments received by the FBO account.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BankTransferDirection {
    #[serde(rename = "outbound")]
    Outbound,
    #[serde(rename = "inbound")]
    Inbound,
    #[serde(rename = "null")]
    Null,

}

impl ToString for BankTransferDirection {
    fn to_string(&self) -> String {
        match self {
            Self::Outbound => String::from("outbound"),
            Self::Inbound => String::from("inbound"),
            Self::Null => String::from("null"),
        }
    }
}

impl Default for BankTransferDirection {
    fn default() -> BankTransferDirection {
        Self::Outbound
    }
}




