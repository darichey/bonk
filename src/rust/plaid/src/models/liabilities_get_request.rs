/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LiabilitiesGetRequest : LiabilitiesGetRequest defines the request schema for `/liabilities/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiabilitiesGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The access token associated with the Item data is being requested for.
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::LiabilitiesGetRequestOptions>>,
}

impl LiabilitiesGetRequest {
    /// LiabilitiesGetRequest defines the request schema for `/liabilities/get`
    pub fn new(access_token: String) -> LiabilitiesGetRequest {
        LiabilitiesGetRequest {
            client_id: None,
            secret: None,
            access_token,
            options: None,
        }
    }
}

