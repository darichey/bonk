/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProcessorTransactionsGetRequestOptions : An optional object to be used with the request. If specified, `options` must not be `null`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProcessorTransactionsGetRequestOptions {
    /// The number of transactions to fetch.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// The number of transactions to skip. The default value is 0.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Include the raw unparsed transaction description from the financial institution.
    #[serde(rename = "include_original_description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub include_original_description: Option<Option<bool>>,
    /// Personal finance categories are now returned by default.
    #[serde(rename = "include_personal_finance_category_beta", skip_serializing_if = "Option::is_none")]
    pub include_personal_finance_category_beta: Option<bool>,
    /// Personal finance categories are now returned by default.
    #[serde(rename = "include_personal_finance_category", skip_serializing_if = "Option::is_none")]
    pub include_personal_finance_category: Option<bool>,
    /// Counterparties and extra merchant fields are now returned by default.
    #[serde(rename = "include_logo_and_counterparty_beta", skip_serializing_if = "Option::is_none")]
    pub include_logo_and_counterparty_beta: Option<bool>,
}

impl ProcessorTransactionsGetRequestOptions {
    /// An optional object to be used with the request. If specified, `options` must not be `null`.
    pub fn new() -> ProcessorTransactionsGetRequestOptions {
        ProcessorTransactionsGetRequestOptions {
            count: None,
            offset: None,
            include_original_description: None,
            include_personal_finance_category_beta: None,
            include_personal_finance_category: None,
            include_logo_and_counterparty_beta: None,
        }
    }
}


