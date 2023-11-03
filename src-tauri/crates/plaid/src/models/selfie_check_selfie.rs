/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SelfieCheckSelfie : Captures and analysis from a user's selfie.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SelfieCheckSelfie {
    #[serde(rename = "status")]
    pub status: crate::models::SelfieStatus,
    /// The `attempt` field begins with 1 and increments with each subsequent selfie upload.
    #[serde(rename = "attempt")]
    pub attempt: i32,
    #[serde(rename = "capture")]
    pub capture: crate::models::SelfieCapture,
    #[serde(rename = "analysis")]
    pub analysis: crate::models::SelfieAnalysis,
}

impl SelfieCheckSelfie {
    /// Captures and analysis from a user's selfie.
    pub fn new(status: crate::models::SelfieStatus, attempt: i32, capture: crate::models::SelfieCapture, analysis: crate::models::SelfieAnalysis) -> SelfieCheckSelfie {
        SelfieCheckSelfie {
            status,
            attempt,
            capture,
            analysis,
        }
    }
}


