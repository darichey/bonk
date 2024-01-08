/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsRecurringCreateInput : TransactionsRecurringCreateInput defines a single input to the `/transactions/recurring/streams/create` endpoint.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionsRecurringCreateInput {
    /// IDs of all the transactions that will be merged into one stream. If any transaction currently exists in another stream, it will be removed from the other stream.
    #[serde(rename = "transaction_ids", skip_serializing_if = "Option::is_none")]
    pub transaction_ids: Option<Vec<String>>,
}

impl TransactionsRecurringCreateInput {
    /// TransactionsRecurringCreateInput defines a single input to the `/transactions/recurring/streams/create` endpoint.
    pub fn new() -> TransactionsRecurringCreateInput {
        TransactionsRecurringCreateInput {
            transaction_ids: None,
        }
    }
}


