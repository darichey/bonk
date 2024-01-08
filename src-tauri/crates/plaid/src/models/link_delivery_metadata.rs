/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkDeliveryMetadata : Information related to the related to the delivery of the link session to users



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkDeliveryMetadata {
    #[serde(rename = "communication_method", skip_serializing_if = "Option::is_none")]
    pub communication_method: Option<crate::models::LinkDeliveryWebhookCommunicationMethod>,
    #[serde(rename = "delivery_status", skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<crate::models::LinkDeliveryWebhookDeliveryStatus>,
}

impl LinkDeliveryMetadata {
    /// Information related to the related to the delivery of the link session to users
    pub fn new() -> LinkDeliveryMetadata {
        LinkDeliveryMetadata {
            communication_method: None,
            delivery_status: None,
        }
    }
}


