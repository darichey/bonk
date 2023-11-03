/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxTransferRefundSimulateResponse : Defines the response schema for `/sandbox/transfer/refund/simulate`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SandboxTransferRefundSimulateResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl SandboxTransferRefundSimulateResponse {
    /// Defines the response schema for `/sandbox/transfer/refund/simulate`
    pub fn new(request_id: String) -> SandboxTransferRefundSimulateResponse {
        SandboxTransferRefundSimulateResponse {
            request_id,
        }
    }
}


