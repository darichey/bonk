/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemProductReadyWebhook : Fired once Plaid calculates income from an Item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemProductReadyWebhook {
    /// `INCOME`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `PRODUCT_READY`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error: Option<Option<crate::models::PlaidError>>,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl ItemProductReadyWebhook {
    /// Fired once Plaid calculates income from an Item.
    pub fn new(webhook_type: String, webhook_code: String, item_id: String, environment: crate::models::WebhookEnvironmentValues) -> ItemProductReadyWebhook {
        ItemProductReadyWebhook {
            webhook_type,
            webhook_code,
            item_id,
            error: None,
            environment,
        }
    }
}


