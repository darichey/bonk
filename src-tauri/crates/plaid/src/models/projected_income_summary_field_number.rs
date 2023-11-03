/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProjectedIncomeSummaryFieldNumber : The employee's estimated annual salary, as derived from information reported on the paystub.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectedIncomeSummaryFieldNumber {
    /// The value of the field.
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(rename = "verification_status")]
    pub verification_status: crate::models::VerificationStatus,
}

impl ProjectedIncomeSummaryFieldNumber {
    /// The employee's estimated annual salary, as derived from information reported on the paystub.
    pub fn new(value: f64, verification_status: crate::models::VerificationStatus) -> ProjectedIncomeSummaryFieldNumber {
        ProjectedIncomeSummaryFieldNumber {
            value,
            verification_status,
        }
    }
}


