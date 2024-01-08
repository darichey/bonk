/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsEnrichRequestOptions : An optional object to be used with the request.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionsEnrichRequestOptions {
    /// Include `legacy_category` and `legacy_category_id` in the response (in addition to the default `personal_finance_category`).  Categories are based on Plaid's legacy taxonomy. For a full list of legacy categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).
    #[serde(rename = "include_legacy_category", skip_serializing_if = "Option::is_none")]
    pub include_legacy_category: Option<bool>,
}

impl TransactionsEnrichRequestOptions {
    /// An optional object to be used with the request.
    pub fn new() -> TransactionsEnrichRequestOptions {
        TransactionsEnrichRequestOptions {
            include_legacy_category: None,
        }
    }
}


