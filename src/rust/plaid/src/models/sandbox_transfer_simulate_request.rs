/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxTransferSimulateRequest : Defines the request schema for `/sandbox/transfer/simulate`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SandboxTransferSimulateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Plaid’s unique identifier for a transfer.
    #[serde(rename = "transfer_id")]
    pub transfer_id: String,
    /// Plaid’s unique identifier for a test clock. If provided, the event to be simulated is created at the `virtual_time` on the provided `test_clock`.
    #[serde(rename = "test_clock_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub test_clock_id: Option<Option<String>>,
    /// The asynchronous event to be simulated. May be: `posted`, `settled`, `failed`, `funds_available`, or `returned`.  An error will be returned if the event type is incompatible with the current transfer status. Compatible status --> event type transitions include:  `pending` --> `failed`  `pending` --> `posted`  `posted` --> `returned`  `posted` --> `settled`  `settled` --> `funds_available` (only applicable to ACH debits.) 
    #[serde(rename = "event_type")]
    pub event_type: String,
    #[serde(rename = "failure_reason", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<Option<crate::models::TransferFailure>>,
    /// The webhook URL to which a `TRANSFER_EVENTS_UPDATE` webhook should be sent.
    #[serde(rename = "webhook", skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}

impl SandboxTransferSimulateRequest {
    /// Defines the request schema for `/sandbox/transfer/simulate`
    pub fn new(transfer_id: String, event_type: String) -> SandboxTransferSimulateRequest {
        SandboxTransferSimulateRequest {
            client_id: None,
            secret: None,
            transfer_id,
            test_clock_id: None,
            event_type,
            failure_reason: None,
            webhook: None,
        }
    }
}


