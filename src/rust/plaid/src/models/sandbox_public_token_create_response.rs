/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxPublicTokenCreateResponse : SandboxPublicTokenCreateResponse defines the response schema for `/sandbox/public_token/create`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SandboxPublicTokenCreateResponse {
    /// A public token that can be exchanged for an access token using `/item/public_token/exchange`
    #[serde(rename = "public_token")]
    pub public_token: String,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl SandboxPublicTokenCreateResponse {
    /// SandboxPublicTokenCreateResponse defines the response schema for `/sandbox/public_token/create`
    pub fn new(public_token: String, request_id: String) -> SandboxPublicTokenCreateResponse {
        SandboxPublicTokenCreateResponse {
            public_token,
            request_id,
        }
    }
}


