/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxProcessorTokenCreateResponse : SandboxProcessorTokenCreateResponse defines the response schema for `/sandbox/processor_token/create`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SandboxProcessorTokenCreateResponse {
    /// A processor token that can be used to call the `/processor/` endpoints.
    #[serde(rename = "processor_token")]
    pub processor_token: String,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl SandboxProcessorTokenCreateResponse {
    /// SandboxProcessorTokenCreateResponse defines the response schema for `/sandbox/processor_token/create`
    pub fn new(processor_token: String, request_id: String) -> SandboxProcessorTokenCreateResponse {
        SandboxProcessorTokenCreateResponse {
            processor_token,
            request_id,
        }
    }
}


