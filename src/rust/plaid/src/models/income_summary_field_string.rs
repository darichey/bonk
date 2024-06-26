/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IncomeSummaryFieldString : Data about the income summary



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncomeSummaryFieldString {
    /// The value of the field.
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "verification_status")]
    pub verification_status: crate::models::VerificationStatus,
}

impl IncomeSummaryFieldString {
    /// Data about the income summary
    pub fn new(value: String, verification_status: crate::models::VerificationStatus) -> IncomeSummaryFieldString {
        IncomeSummaryFieldString {
            value,
            verification_status,
        }
    }
}


