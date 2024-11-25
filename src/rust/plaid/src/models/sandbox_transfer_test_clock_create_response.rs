/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxTransferTestClockCreateResponse : Defines the response schema for `/sandbox/transfer/test_clock/create`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SandboxTransferTestClockCreateResponse {
    #[serde(rename = "test_clock")]
    pub test_clock: crate::models::TransferTestClock,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl SandboxTransferTestClockCreateResponse {
    /// Defines the response schema for `/sandbox/transfer/test_clock/create`
    pub fn new(test_clock: crate::models::TransferTestClock, request_id: String) -> SandboxTransferTestClockCreateResponse {
        SandboxTransferTestClockCreateResponse {
            test_clock,
            request_id,
        }
    }
}


