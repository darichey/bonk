/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProcessorTransactionsGetResponse : ProcessorTransactionsGetResponse defines the response schema for `/processor/transactions/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessorTransactionsGetResponse {
    #[serde(rename = "account")]
    pub account: crate::models::AccountBase,
    /// An array containing transactions from the account. Transactions are returned in reverse chronological order, with the most recent at the beginning of the array. The maximum number of transactions returned is determined by the `count` parameter.
    #[serde(rename = "transactions")]
    pub transactions: Vec<crate::models::Transaction>,
    /// The total number of transactions available within the date range specified. If `total_transactions` is larger than the size of the `transactions` array, more transactions are available and can be fetched via manipulating the `offset` parameter.
    #[serde(rename = "total_transactions")]
    pub total_transactions: i32,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl ProcessorTransactionsGetResponse {
    /// ProcessorTransactionsGetResponse defines the response schema for `/processor/transactions/get`
    pub fn new(account: crate::models::AccountBase, transactions: Vec<crate::models::Transaction>, total_transactions: i32, request_id: String) -> ProcessorTransactionsGetResponse {
        ProcessorTransactionsGetResponse {
            account,
            transactions,
            total_transactions,
            request_id,
        }
    }
}

