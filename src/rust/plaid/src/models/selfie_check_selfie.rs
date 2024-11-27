/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SelfieCheckSelfie : Captures and analysis from a user's selfie.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelfieCheckSelfie {
    #[serde(rename = "status")]
    pub status: models::SelfieStatus,
    /// The `attempt` field begins with 1 and increments with each subsequent selfie upload.
    #[serde(rename = "attempt")]
    pub attempt: i32,
    #[serde(rename = "capture")]
    pub capture: models::SelfieCapture,
    #[serde(rename = "analysis")]
    pub analysis: models::SelfieAnalysis,
}

impl SelfieCheckSelfie {
    /// Captures and analysis from a user's selfie.
    pub fn new(status: models::SelfieStatus, attempt: i32, capture: models::SelfieCapture, analysis: models::SelfieAnalysis) -> SelfieCheckSelfie {
        SelfieCheckSelfie {
            status,
            attempt,
            capture,
            analysis,
        }
    }
}

