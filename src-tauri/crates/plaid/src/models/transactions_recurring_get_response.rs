/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsRecurringGetResponse : TransactionsRecurringGetResponse defines the response schema for `/transactions/recurring/get`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionsRecurringGetResponse {
    /// An array of depository transaction streams.
    #[serde(rename = "inflow_streams")]
    pub inflow_streams: Vec<crate::models::TransactionStream>,
    /// An array of expense transaction streams.
    #[serde(rename = "outflow_streams")]
    pub outflow_streams: Vec<crate::models::TransactionStream>,
    /// Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the last time transaction streams for the given account were updated on
    #[serde(rename = "updated_datetime")]
    pub updated_datetime: String,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransactionsRecurringGetResponse {
    /// TransactionsRecurringGetResponse defines the response schema for `/transactions/recurring/get`
    pub fn new(inflow_streams: Vec<crate::models::TransactionStream>, outflow_streams: Vec<crate::models::TransactionStream>, updated_datetime: String, request_id: String) -> TransactionsRecurringGetResponse {
        TransactionsRecurringGetResponse {
            inflow_streams,
            outflow_streams,
            updated_datetime,
            request_id,
        }
    }
}


