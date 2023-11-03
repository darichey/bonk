/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditPayrollIncomeGetResponse : Defines the response body for `/credit/payroll_income/get`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditPayrollIncomeGetResponse {
    /// Array of payroll items.
    #[serde(rename = "items")]
    pub items: Vec<crate::models::PayrollItem>,
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error: Option<Option<crate::models::PlaidError>>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl CreditPayrollIncomeGetResponse {
    /// Defines the response body for `/credit/payroll_income/get`.
    pub fn new(items: Vec<crate::models::PayrollItem>, request_id: String) -> CreditPayrollIncomeGetResponse {
        CreditPayrollIncomeGetResponse {
            items,
            error: None,
            request_id,
        }
    }
}


