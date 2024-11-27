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

/// DocumentaryVerificationDocument : Images, extracted data, and analysis from a user's identity document
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentaryVerificationDocument {
    #[serde(rename = "status")]
    pub status: models::DocumentStatus,
    /// The `attempt` field begins with 1 and increments with each subsequent document upload.
    #[serde(rename = "attempt")]
    pub attempt: i32,
    #[serde(rename = "images")]
    pub images: models::PhysicalDocumentImages,
    #[serde(rename = "extracted_data", deserialize_with = "Option::deserialize")]
    pub extracted_data: Option<models::PhysicalDocumentExtractedData>,
    #[serde(rename = "analysis")]
    pub analysis: models::DocumentAnalysis,
    /// An ISO8601 formatted timestamp.
    #[serde(rename = "redacted_at", deserialize_with = "Option::deserialize")]
    pub redacted_at: Option<String>,
}

impl DocumentaryVerificationDocument {
    /// Images, extracted data, and analysis from a user's identity document
    pub fn new(status: models::DocumentStatus, attempt: i32, images: models::PhysicalDocumentImages, extracted_data: Option<models::PhysicalDocumentExtractedData>, analysis: models::DocumentAnalysis, redacted_at: Option<String>) -> DocumentaryVerificationDocument {
        DocumentaryVerificationDocument {
            status,
            attempt,
            images,
            extracted_data,
            analysis,
            redacted_at,
        }
    }
}

