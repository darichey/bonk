/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferDiligenceStatus : Originator’s diligence status.

/// Originator’s diligence status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferDiligenceStatus {
    #[serde(rename = "not_submitted")]
    NotSubmitted,
    #[serde(rename = "submitted")]
    Submitted,
    #[serde(rename = "under_review")]
    UnderReview,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "denied")]
    Denied,

}

impl ToString for TransferDiligenceStatus {
    fn to_string(&self) -> String {
        match self {
            Self::NotSubmitted => String::from("not_submitted"),
            Self::Submitted => String::from("submitted"),
            Self::UnderReview => String::from("under_review"),
            Self::Approved => String::from("approved"),
            Self::Denied => String::from("denied"),
        }
    }
}

impl Default for TransferDiligenceStatus {
    fn default() -> TransferDiligenceStatus {
        Self::NotSubmitted
    }
}




