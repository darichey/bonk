/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxTransferTestClockAdvanceRequest : Defines the request schema for `/sandbox/transfer/test_clock/advance`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SandboxTransferTestClockAdvanceRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Plaid’s unique identifier for a test clock.
    #[serde(rename = "test_clock_id")]
    pub test_clock_id: String,
    /// The virtual timestamp on the test clock. This will be of the form `2006-01-02T15:04:05Z`.
    #[serde(rename = "new_virtual_time")]
    pub new_virtual_time: String,
}

impl SandboxTransferTestClockAdvanceRequest {
    /// Defines the request schema for `/sandbox/transfer/test_clock/advance`
    pub fn new(test_clock_id: String, new_virtual_time: String) -> SandboxTransferTestClockAdvanceRequest {
        SandboxTransferTestClockAdvanceRequest {
            client_id: None,
            secret: None,
            test_clock_id,
            new_virtual_time,
        }
    }
}


