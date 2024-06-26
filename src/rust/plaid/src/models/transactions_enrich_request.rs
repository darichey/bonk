/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsEnrichRequest : TransactionsEnrichRequest defines the request schema for `/transactions/enrich`.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionsEnrichRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The account type for the requested transactions (either `depository` or `credit`).
    #[serde(rename = "account_type")]
    pub account_type: String,
    /// An array of transaction objects to be enriched by Plaid. Maximum of 100 transactions per request.
    #[serde(rename = "transactions")]
    pub transactions: Vec<crate::models::ClientProvidedTransaction>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::TransactionsEnrichRequestOptions>>,
}

impl TransactionsEnrichRequest {
    /// TransactionsEnrichRequest defines the request schema for `/transactions/enrich`.
    pub fn new(account_type: String, transactions: Vec<crate::models::ClientProvidedTransaction>) -> TransactionsEnrichRequest {
        TransactionsEnrichRequest {
            client_id: None,
            secret: None,
            account_type,
            transactions,
            options: None,
        }
    }
}


