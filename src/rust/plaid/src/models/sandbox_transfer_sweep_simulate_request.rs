/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxTransferSweepSimulateRequest : Defines the request schema for `/sandbox/transfer/sweep/simulate`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SandboxTransferSweepSimulateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Plaid’s unique identifier for a test clock. If provided, the sweep to be simulated is created on the day of the `virtual_time` on the `test_clock`. If the date of `virtual_time` is on weekend or a federal holiday, the next available banking day is used.
    #[serde(rename = "test_clock_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub test_clock_id: Option<Option<String>>,
}

impl SandboxTransferSweepSimulateRequest {
    /// Defines the request schema for `/sandbox/transfer/sweep/simulate`
    pub fn new() -> SandboxTransferSweepSimulateRequest {
        SandboxTransferSweepSimulateRequest {
            client_id: None,
            secret: None,
            test_clock_id: None,
        }
    }
}

