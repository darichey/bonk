/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IncomeVerificationCreateResponse : IncomeVerificationCreateResponse defines the response schema for `/income/verification/create`.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncomeVerificationCreateResponse {
    /// ID of the verification. This ID is persisted throughout the lifetime of the verification.
    #[serde(rename = "income_verification_id")]
    pub income_verification_id: String,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl IncomeVerificationCreateResponse {
    /// IncomeVerificationCreateResponse defines the response schema for `/income/verification/create`.
    pub fn new(income_verification_id: String, request_id: String) -> IncomeVerificationCreateResponse {
        IncomeVerificationCreateResponse {
            income_verification_id,
            request_id,
        }
    }
}


