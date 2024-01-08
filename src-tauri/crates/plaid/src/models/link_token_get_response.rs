/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenGetResponse : LinkTokenGetResponse defines the response schema for `/link/token/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokenGetResponse {
    /// A `link_token`, which can be supplied to Link in order to initialize it and receive a `public_token`, which can be exchanged for an `access_token`.
    #[serde(rename = "link_token")]
    pub link_token: String,
    /// The creation timestamp for the `link_token`, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    #[serde(rename = "created_at", deserialize_with = "Option::deserialize")]
    pub created_at: Option<String>,
    /// The expiration timestamp for the `link_token`, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    #[serde(rename = "expiration", deserialize_with = "Option::deserialize")]
    pub expiration: Option<String>,
    /// Information about Link sessions created using this `link_token`. This field will only be present if your client is enabled for Hosted Link (beta). Session data will be provided for up to six hours after the session has ended.
    #[serde(rename = "link_sessions", skip_serializing_if = "Option::is_none")]
    pub link_sessions: Option<Vec<crate::models::LinkTokenGetSessionsResponse>>,
    #[serde(rename = "metadata")]
    pub metadata: crate::models::LinkTokenGetMetadataResponse,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl LinkTokenGetResponse {
    /// LinkTokenGetResponse defines the response schema for `/link/token/get`
    pub fn new(link_token: String, created_at: Option<String>, expiration: Option<String>, metadata: crate::models::LinkTokenGetMetadataResponse, request_id: String) -> LinkTokenGetResponse {
        LinkTokenGetResponse {
            link_token,
            created_at,
            expiration,
            link_sessions: None,
            metadata,
            request_id,
        }
    }
}


