/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemAccessTokenInvalidateRequest : ItemAccessTokenInvalidateRequest defines the request schema for `/item/access_token/invalidate`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemAccessTokenInvalidateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The access token associated with the Item data is being requested for.
    #[serde(rename = "access_token")]
    pub access_token: String,
}

impl ItemAccessTokenInvalidateRequest {
    /// ItemAccessTokenInvalidateRequest defines the request schema for `/item/access_token/invalidate`
    pub fn new(access_token: String) -> ItemAccessTokenInvalidateRequest {
        ItemAccessTokenInvalidateRequest {
            client_id: None,
            secret: None,
            access_token,
        }
    }
}


