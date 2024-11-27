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

/// PhysicalDocumentExtractedDataAnalysis : Analysis of the data extracted from the submitted document.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhysicalDocumentExtractedDataAnalysis {
    #[serde(rename = "name")]
    pub name: models::DocumentNameMatchCode,
    #[serde(rename = "date_of_birth")]
    pub date_of_birth: models::DocumentDateOfBirthMatchCode,
    #[serde(rename = "expiration_date")]
    pub expiration_date: models::ExpirationDate,
    #[serde(rename = "issuing_country")]
    pub issuing_country: models::IssuingCountry,
}

impl PhysicalDocumentExtractedDataAnalysis {
    /// Analysis of the data extracted from the submitted document.
    pub fn new(name: models::DocumentNameMatchCode, date_of_birth: models::DocumentDateOfBirthMatchCode, expiration_date: models::ExpirationDate, issuing_country: models::IssuingCountry) -> PhysicalDocumentExtractedDataAnalysis {
        PhysicalDocumentExtractedDataAnalysis {
            name,
            date_of_birth,
            expiration_date,
            issuing_country,
        }
    }
}

