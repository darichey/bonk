/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxItemSetVerificationStatusResponse : SandboxItemSetVerificationStatusResponse defines the response schema for `/sandbox/item/set_verification_status`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SandboxItemSetVerificationStatusResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl SandboxItemSetVerificationStatusResponse {
    /// SandboxItemSetVerificationStatusResponse defines the response schema for `/sandbox/item/set_verification_status`
    pub fn new(request_id: String) -> SandboxItemSetVerificationStatusResponse {
        SandboxItemSetVerificationStatusResponse {
            request_id,
        }
    }
}


