/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProcessorTransactionsRefreshResponse : ProcessorTransactionsRefreshResponse defines the response schema for `/processor/transactions/refresh`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProcessorTransactionsRefreshResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl ProcessorTransactionsRefreshResponse {
    /// ProcessorTransactionsRefreshResponse defines the response schema for `/processor/transactions/refresh`
    pub fn new(request_id: String) -> ProcessorTransactionsRefreshResponse {
        ProcessorTransactionsRefreshResponse {
            request_id,
        }
    }
}


