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

/// IncomeSummaryFieldNumber : Field number for income summary
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncomeSummaryFieldNumber {
    /// The value of the field.
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(rename = "verification_status")]
    pub verification_status: models::VerificationStatus,
}

impl IncomeSummaryFieldNumber {
    /// Field number for income summary
    pub fn new(value: f64, verification_status: models::VerificationStatus) -> IncomeSummaryFieldNumber {
        IncomeSummaryFieldNumber {
            value,
            verification_status,
        }
    }
}

