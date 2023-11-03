/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxTransferLedgerSimulateAvailableRequest : Defines the request schema for `/sandbox/transfer/ledger/simulate_available`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SandboxTransferLedgerSimulateAvailableRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Plaid’s unique identifier for a test clock. If provided, only the pending balance that is due before the `virtual_timestamp` on the test clock will be converted.
    #[serde(rename = "test_clock_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub test_clock_id: Option<Option<String>>,
}

impl SandboxTransferLedgerSimulateAvailableRequest {
    /// Defines the request schema for `/sandbox/transfer/ledger/simulate_available`
    pub fn new() -> SandboxTransferLedgerSimulateAvailableRequest {
        SandboxTransferLedgerSimulateAvailableRequest {
            client_id: None,
            secret: None,
            test_clock_id: None,
        }
    }
}


