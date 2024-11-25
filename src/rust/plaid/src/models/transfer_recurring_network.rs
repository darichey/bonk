/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferRecurringNetwork : Networks eligible for recurring transfers.

/// Networks eligible for recurring transfers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferRecurringNetwork {
    #[serde(rename = "ach")]
    Ach,
    #[serde(rename = "same-day-ach")]
    SameDayAch,
    #[serde(rename = "rtp")]
    Rtp,

}

impl ToString for TransferRecurringNetwork {
    fn to_string(&self) -> String {
        match self {
            Self::Ach => String::from("ach"),
            Self::SameDayAch => String::from("same-day-ach"),
            Self::Rtp => String::from("rtp"),
        }
    }
}

impl Default for TransferRecurringNetwork {
    fn default() -> TransferRecurringNetwork {
        Self::Ach
    }
}




