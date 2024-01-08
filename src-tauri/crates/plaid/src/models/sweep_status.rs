/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SweepStatus : The status of a sweep transfer  `\"pending\"` - The sweep is currently pending `\"posted\"` - The sweep has been posted `\"settled\"` - The sweep has settled `\"returned\"` - The sweep has been returned `\"failed\"` - The sweep has failed

/// The status of a sweep transfer  `\"pending\"` - The sweep is currently pending `\"posted\"` - The sweep has been posted `\"settled\"` - The sweep has settled `\"returned\"` - The sweep has been returned `\"failed\"` - The sweep has failed
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SweepStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "posted")]
    Posted,
    #[serde(rename = "settled")]
    Settled,
    #[serde(rename = "returned")]
    Returned,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "null")]
    Null,

}

impl ToString for SweepStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Pending => String::from("pending"),
            Self::Posted => String::from("posted"),
            Self::Settled => String::from("settled"),
            Self::Returned => String::from("returned"),
            Self::Failed => String::from("failed"),
            Self::Null => String::from("null"),
        }
    }
}

impl Default for SweepStatus {
    fn default() -> SweepStatus {
        Self::Pending
    }
}




