/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkDeliveryVerificationStatus : Indicates an Item's micro-deposit-based verification status.

/// Indicates an Item's micro-deposit-based verification status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LinkDeliveryVerificationStatus {
    #[serde(rename = "automatically_verified")]
    AutomaticallyVerified,
    #[serde(rename = "pending_automatic_verification")]
    PendingAutomaticVerification,
    #[serde(rename = "pending_manual_verification")]
    PendingManualVerification,
    #[serde(rename = "manually_verified")]
    ManuallyVerified,
    #[serde(rename = "verification_expired")]
    VerificationExpired,
    #[serde(rename = "verification_failed")]
    VerificationFailed,
    #[serde(rename = "database_matched")]
    DatabaseMatched,

}

impl ToString for LinkDeliveryVerificationStatus {
    fn to_string(&self) -> String {
        match self {
            Self::AutomaticallyVerified => String::from("automatically_verified"),
            Self::PendingAutomaticVerification => String::from("pending_automatic_verification"),
            Self::PendingManualVerification => String::from("pending_manual_verification"),
            Self::ManuallyVerified => String::from("manually_verified"),
            Self::VerificationExpired => String::from("verification_expired"),
            Self::VerificationFailed => String::from("verification_failed"),
            Self::DatabaseMatched => String::from("database_matched"),
        }
    }
}

impl Default for LinkDeliveryVerificationStatus {
    fn default() -> LinkDeliveryVerificationStatus {
        Self::AutomaticallyVerified
    }
}




