/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EmploymentVerificationGetResponse : EmploymentVerificationGetResponse defines the response schema for `/employment/verification/get`.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmploymentVerificationGetResponse {
    /// A list of employment verification summaries.
    #[serde(rename = "employments")]
    pub employments: Vec<crate::models::EmploymentVerification>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl EmploymentVerificationGetResponse {
    /// EmploymentVerificationGetResponse defines the response schema for `/employment/verification/get`.
    pub fn new(employments: Vec<crate::models::EmploymentVerification>, request_id: String) -> EmploymentVerificationGetResponse {
        EmploymentVerificationGetResponse {
            employments,
            request_id,
        }
    }
}


