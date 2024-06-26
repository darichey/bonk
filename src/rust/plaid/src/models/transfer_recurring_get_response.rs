/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferRecurringGetResponse : Defines the response schema for `/transfer/recurring/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferRecurringGetResponse {
    #[serde(rename = "recurring_transfer")]
    pub recurring_transfer: crate::models::RecurringTransfer,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransferRecurringGetResponse {
    /// Defines the response schema for `/transfer/recurring/get`
    pub fn new(recurring_transfer: crate::models::RecurringTransfer, request_id: String) -> TransferRecurringGetResponse {
        TransferRecurringGetResponse {
            recurring_transfer,
            request_id,
        }
    }
}


