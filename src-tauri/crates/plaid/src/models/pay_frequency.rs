/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PayFrequency : The frequency of the pay period.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayFrequency {
    #[serde(rename = "value")]
    pub value: crate::models::PayFrequencyValue,
    #[serde(rename = "verification_status")]
    pub verification_status: crate::models::VerificationStatus,
}

impl PayFrequency {
    /// The frequency of the pay period.
    pub fn new(value: crate::models::PayFrequencyValue, verification_status: crate::models::VerificationStatus) -> PayFrequency {
        PayFrequency {
            value,
            verification_status,
        }
    }
}


