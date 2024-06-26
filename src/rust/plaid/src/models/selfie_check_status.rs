/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SelfieCheckStatus : The outcome status for the associated Identity Verification attempt's `selfie_check` step. This field will always have the same value as `steps.selfie_check`.

/// The outcome status for the associated Identity Verification attempt's `selfie_check` step. This field will always have the same value as `steps.selfie_check`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SelfieCheckStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,

}

impl ToString for SelfieCheckStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Success => String::from("success"),
            Self::Failed => String::from("failed"),
        }
    }
}

impl Default for SelfieCheckStatus {
    fn default() -> SelfieCheckStatus {
        Self::Success
    }
}




