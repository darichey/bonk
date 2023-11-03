/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SelfieStatus : An outcome status for this specific selfie. Distinct from the overall `selfie_check.status` that summarizes the verification outcome from one or more selfies.

/// An outcome status for this specific selfie. Distinct from the overall `selfie_check.status` that summarizes the verification outcome from one or more selfies.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SelfieStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,

}

impl ToString for SelfieStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Success => String::from("success"),
            Self::Failed => String::from("failed"),
        }
    }
}

impl Default for SelfieStatus {
    fn default() -> SelfieStatus {
        Self::Success
    }
}




