/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkDeliveryCallbackWebhook : Webhook containing metadata proxied over from Link callback e.g `onEvent`, `onExit`, `onSuccess`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkDeliveryCallbackWebhook {
    /// `LINK_DELIVERY`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `LINK_CALLBACK`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The ID of the Hosted Link session.
    #[serde(rename = "link_delivery_session_id")]
    pub link_delivery_session_id: String,
    /// Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error: Option<Option<crate::models::PlaidError>>,
    #[serde(rename = "link_callback_metadata")]
    pub link_callback_metadata: crate::models::LinkCallbackMetadata,
}

impl LinkDeliveryCallbackWebhook {
    /// Webhook containing metadata proxied over from Link callback e.g `onEvent`, `onExit`, `onSuccess`.
    pub fn new(webhook_type: String, webhook_code: String, link_delivery_session_id: String, timestamp: String, link_callback_metadata: crate::models::LinkCallbackMetadata) -> LinkDeliveryCallbackWebhook {
        LinkDeliveryCallbackWebhook {
            webhook_type,
            webhook_code,
            link_delivery_session_id,
            timestamp,
            error: None,
            link_callback_metadata,
        }
    }
}


