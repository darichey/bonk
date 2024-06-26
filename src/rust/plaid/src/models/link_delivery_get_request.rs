/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkDeliveryGetRequest : LinkDeliveryGetRequest defines the request schema for `/link_delivery/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkDeliveryGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The ID for the Hosted Link session from a previous invocation of `/link_delivery/create`.
    #[serde(rename = "link_delivery_session_id")]
    pub link_delivery_session_id: String,
}

impl LinkDeliveryGetRequest {
    /// LinkDeliveryGetRequest defines the request schema for `/link_delivery/get`
    pub fn new(link_delivery_session_id: String) -> LinkDeliveryGetRequest {
        LinkDeliveryGetRequest {
            client_id: None,
            secret: None,
            link_delivery_session_id,
        }
    }
}


