/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferAuthorizationCreateResponse : Defines the response schema for `/transfer/authorization/create`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferAuthorizationCreateResponse {
    #[serde(rename = "authorization")]
    pub authorization: crate::models::TransferAuthorization,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransferAuthorizationCreateResponse {
    /// Defines the response schema for `/transfer/authorization/create`
    pub fn new(authorization: crate::models::TransferAuthorization, request_id: String) -> TransferAuthorizationCreateResponse {
        TransferAuthorizationCreateResponse {
            authorization,
            request_id,
        }
    }
}


