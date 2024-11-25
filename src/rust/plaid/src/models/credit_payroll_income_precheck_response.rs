/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditPayrollIncomePrecheckResponse : Defines the response schema for `/credit/payroll_income/precheck`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditPayrollIncomePrecheckResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
    #[serde(rename = "confidence")]
    pub confidence: crate::models::IncomeVerificationPrecheckConfidence,
}

impl CreditPayrollIncomePrecheckResponse {
    /// Defines the response schema for `/credit/payroll_income/precheck`.
    pub fn new(request_id: String, confidence: crate::models::IncomeVerificationPrecheckConfidence) -> CreditPayrollIncomePrecheckResponse {
        CreditPayrollIncomePrecheckResponse {
            request_id,
            confidence,
        }
    }
}


