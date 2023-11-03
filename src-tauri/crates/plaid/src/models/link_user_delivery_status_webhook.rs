/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkUserDeliveryStatusWebhook : Webhook indicating that the status of the delivery of the hosted link session to a user



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkUserDeliveryStatusWebhook {
    /// `LINK_DELIVERY`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `DELIVERY_STATUS`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The ID of the Hosted Link session.
    #[serde(rename = "link_delivery_session_id")]
    pub link_delivery_session_id: String,
    /// Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "link_delivery_metadata")]
    pub link_delivery_metadata: crate::models::LinkDeliveryMetadata,
}

impl LinkUserDeliveryStatusWebhook {
    /// Webhook indicating that the status of the delivery of the hosted link session to a user
    pub fn new(webhook_type: String, webhook_code: String, link_delivery_session_id: String, timestamp: String, link_delivery_metadata: crate::models::LinkDeliveryMetadata) -> LinkUserDeliveryStatusWebhook {
        LinkUserDeliveryStatusWebhook {
            webhook_type,
            webhook_code,
            link_delivery_session_id,
            timestamp,
            link_delivery_metadata,
        }
    }
}


