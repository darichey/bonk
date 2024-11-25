/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsGetResponse : TransactionsGetResponse defines the response schema for `/transactions/get`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionsGetResponse {
    /// An array containing the `accounts` associated with the Item for which transactions are being returned. Each transaction can be mapped to its corresponding account via the `account_id` field.
    #[serde(rename = "accounts")]
    pub accounts: Vec<crate::models::AccountBase>,
    /// An array containing transactions from the account. Transactions are returned in reverse chronological order, with the most recent at the beginning of the array. The maximum number of transactions returned is determined by the `count` parameter.
    #[serde(rename = "transactions")]
    pub transactions: Vec<crate::models::Transaction>,
    /// The total number of transactions available within the date range specified. If `total_transactions` is larger than the size of the `transactions` array, more transactions are available and can be fetched via manipulating the `offset` parameter.
    #[serde(rename = "total_transactions")]
    pub total_transactions: i32,
    #[serde(rename = "item")]
    pub item: crate::models::Item,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransactionsGetResponse {
    /// TransactionsGetResponse defines the response schema for `/transactions/get`
    pub fn new(accounts: Vec<crate::models::AccountBase>, transactions: Vec<crate::models::Transaction>, total_transactions: i32, item: crate::models::Item, request_id: String) -> TransactionsGetResponse {
        TransactionsGetResponse {
            accounts,
            transactions,
            total_transactions,
            item,
            request_id,
        }
    }
}


