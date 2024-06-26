/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkDeliveryCreateResponse : LinkDeliveryCreateResponse defines the response schema for `/link_delivery/create`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkDeliveryCreateResponse {
    /// The URL to the Hosted Link session, which will be delivered by the specified delivery method.
    #[serde(rename = "link_delivery_url")]
    pub link_delivery_url: String,
    /// The ID for the Hosted Link session. Same as the `link_token` string excluding the \"link-{env}-\" prefix.
    #[serde(rename = "link_delivery_session_id")]
    pub link_delivery_session_id: String,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl LinkDeliveryCreateResponse {
    /// LinkDeliveryCreateResponse defines the response schema for `/link_delivery/create`
    pub fn new(link_delivery_url: String, link_delivery_session_id: String, request_id: String) -> LinkDeliveryCreateResponse {
        LinkDeliveryCreateResponse {
            link_delivery_url,
            link_delivery_session_id,
            request_id,
        }
    }
}


