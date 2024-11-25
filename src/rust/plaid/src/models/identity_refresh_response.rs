/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IdentityRefreshResponse : IdentityRefreshResponse defines the response schema for `/identity/refresh`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IdentityRefreshResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl IdentityRefreshResponse {
    /// IdentityRefreshResponse defines the response schema for `/identity/refresh`
    pub fn new(request_id: String) -> IdentityRefreshResponse {
        IdentityRefreshResponse {
            request_id,
        }
    }
}


