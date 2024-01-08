/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxTransferSimulateResponse : Defines the response schema for `/sandbox/transfer/simulate`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SandboxTransferSimulateResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl SandboxTransferSimulateResponse {
    /// Defines the response schema for `/sandbox/transfer/simulate`
    pub fn new(request_id: String) -> SandboxTransferSimulateResponse {
        SandboxTransferSimulateResponse {
            request_id,
        }
    }
}


