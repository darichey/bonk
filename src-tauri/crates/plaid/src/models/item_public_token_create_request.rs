/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemPublicTokenCreateRequest : ItemPublicTokenCreateRequest defines the request schema for `/item/public_token/create`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemPublicTokenCreateRequest {
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

impl ItemPublicTokenCreateRequest {
    /// ItemPublicTokenCreateRequest defines the request schema for `/item/public_token/create`
    pub fn new(access_token: String) -> ItemPublicTokenCreateRequest {
        ItemPublicTokenCreateRequest {
            client_id: None,
            secret: None,
            access_token,
        }
    }
}


