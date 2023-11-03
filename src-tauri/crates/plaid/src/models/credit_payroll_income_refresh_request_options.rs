/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditPayrollIncomeRefreshRequestOptions : An optional object for `/credit/payroll_income/refresh` request options.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditPayrollIncomeRefreshRequestOptions {
    /// An array of `item_id`s to be refreshed. Each `item_id` should uniquely identify a payroll income item.
    #[serde(rename = "item_ids", skip_serializing_if = "Option::is_none")]
    pub item_ids: Option<Vec<String>>,
    /// The URL where Plaid will send the payroll income refresh webhook.
    #[serde(rename = "webhook", skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}

impl CreditPayrollIncomeRefreshRequestOptions {
    /// An optional object for `/credit/payroll_income/refresh` request options.
    pub fn new() -> CreditPayrollIncomeRefreshRequestOptions {
        CreditPayrollIncomeRefreshRequestOptions {
            item_ids: None,
            webhook: None,
        }
    }
}


