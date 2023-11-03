/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EmployerIncomeSummaryFieldString : The name of the employer, as reported on the paystub.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EmployerIncomeSummaryFieldString {
    /// The value of the field.
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "verification_status")]
    pub verification_status: crate::models::VerificationStatus,
}

impl EmployerIncomeSummaryFieldString {
    /// The name of the employer, as reported on the paystub.
    pub fn new(value: String, verification_status: crate::models::VerificationStatus) -> EmployerIncomeSummaryFieldString {
        EmployerIncomeSummaryFieldString {
            value,
            verification_status,
        }
    }
}


