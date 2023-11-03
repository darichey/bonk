/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsRefreshResponse : TransactionsRefreshResponse defines the response schema for `/transactions/refresh`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionsRefreshResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransactionsRefreshResponse {
    /// TransactionsRefreshResponse defines the response schema for `/transactions/refresh`
    pub fn new(request_id: String) -> TransactionsRefreshResponse {
        TransactionsRefreshResponse {
            request_id,
        }
    }
}


