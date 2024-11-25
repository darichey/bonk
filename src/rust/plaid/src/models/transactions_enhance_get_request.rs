/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsEnhanceGetRequest : TransactionsEnhanceGetRequest defines the request schema for `/transactions/enhance`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionsEnhanceGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The type of account for the requested transactions (`depository` or `credit`).
    #[serde(rename = "account_type")]
    pub account_type: String,
    /// An array of raw transactions to be enhanced.
    #[serde(rename = "transactions")]
    pub transactions: Vec<crate::models::ClientProvidedRawTransaction>,
}

impl TransactionsEnhanceGetRequest {
    /// TransactionsEnhanceGetRequest defines the request schema for `/transactions/enhance`.
    pub fn new(account_type: String, transactions: Vec<crate::models::ClientProvidedRawTransaction>) -> TransactionsEnhanceGetRequest {
        TransactionsEnhanceGetRequest {
            client_id: None,
            secret: None,
            account_type,
            transactions,
        }
    }
}


