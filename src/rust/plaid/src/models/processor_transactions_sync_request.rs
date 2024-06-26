/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProcessorTransactionsSyncRequest : ProcessorTransactionsSyncRequest defines the request schema for `/processor/transactions/sync`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessorTransactionsSyncRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// The processor token obtained from the Plaid integration partner. Processor tokens are in the format: `processor-<environment>-<identifier>`
    #[serde(rename = "processor_token")]
    pub processor_token: String,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The cursor value represents the last update requested. Providing it will cause the response to only return changes after this update. If omitted, the entire history of updates will be returned, starting with the first-added transactions on the item. Note: The upper-bound length of this cursor is 256 characters of base64.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The number of transaction updates to fetch.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::TransactionsSyncRequestOptions>>,
}

impl ProcessorTransactionsSyncRequest {
    /// ProcessorTransactionsSyncRequest defines the request schema for `/processor/transactions/sync`
    pub fn new(processor_token: String) -> ProcessorTransactionsSyncRequest {
        ProcessorTransactionsSyncRequest {
            client_id: None,
            processor_token,
            secret: None,
            cursor: None,
            count: None,
            options: None,
        }
    }
}


