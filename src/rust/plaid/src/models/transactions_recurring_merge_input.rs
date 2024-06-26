/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsRecurringMergeInput : TransactionsRecurringMergeInput defines a single input to the `/transactions/recurring/streams/merge` endpoint.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionsRecurringMergeInput {
    /// IDs of all the streams that will be merged into the first stream. This operation will retain the stream_id of the first stream.
    #[serde(rename = "stream_ids")]
    pub stream_ids: Vec<String>,
}

impl TransactionsRecurringMergeInput {
    /// TransactionsRecurringMergeInput defines a single input to the `/transactions/recurring/streams/merge` endpoint.
    pub fn new(stream_ids: Vec<String>) -> TransactionsRecurringMergeInput {
        TransactionsRecurringMergeInput {
            stream_ids,
        }
    }
}


