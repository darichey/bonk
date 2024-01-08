/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsRecurringMergeResponse : TransactionsRecurringMergeResponse defines the response schema for the `/transactions/recurring/streams/merge` endpoint.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionsRecurringMergeResponse {
    /// Directly modified stream, along with other streams with transactions removed from them as a result of the operation (in no particular order).
    #[serde(rename = "modified_streams")]
    pub modified_streams: Vec<crate::models::TransactionStream>,
    /// The ids of streams that are no longer qualified as recurring transaction streams (in no particular order).
    #[serde(rename = "removed_stream_ids", skip_serializing_if = "Option::is_none")]
    pub removed_stream_ids: Option<Vec<String>>,
}

impl TransactionsRecurringMergeResponse {
    /// TransactionsRecurringMergeResponse defines the response schema for the `/transactions/recurring/streams/merge` endpoint.
    pub fn new(modified_streams: Vec<crate::models::TransactionStream>) -> TransactionsRecurringMergeResponse {
        TransactionsRecurringMergeResponse {
            modified_streams,
            removed_stream_ids: None,
        }
    }
}


