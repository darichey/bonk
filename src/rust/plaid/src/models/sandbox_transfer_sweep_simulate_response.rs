/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxTransferSweepSimulateResponse : Defines the response schema for `/sandbox/transfer/sweep/simulate`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SandboxTransferSweepSimulateResponse {
    #[serde(rename = "sweep", skip_serializing_if = "Option::is_none")]
    pub sweep: Option<Box<crate::models::SimulatedTransferSweep>>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl SandboxTransferSweepSimulateResponse {
    /// Defines the response schema for `/sandbox/transfer/sweep/simulate`
    pub fn new(request_id: String) -> SandboxTransferSweepSimulateResponse {
        SandboxTransferSweepSimulateResponse {
            sweep: None,
            request_id,
        }
    }
}


