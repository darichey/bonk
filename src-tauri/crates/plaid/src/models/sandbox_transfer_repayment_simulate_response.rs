/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxTransferRepaymentSimulateResponse : Defines the response schema for `/sandbox/transfer/repayment/simulate`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SandboxTransferRepaymentSimulateResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl SandboxTransferRepaymentSimulateResponse {
    /// Defines the response schema for `/sandbox/transfer/repayment/simulate`
    pub fn new(request_id: String) -> SandboxTransferRepaymentSimulateResponse {
        SandboxTransferRepaymentSimulateResponse {
            request_id,
        }
    }
}


