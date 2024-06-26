/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HoldingsDefaultUpdateWebhook : Fired when new or updated holdings have been detected on an investment account. The webhook typically fires in response to any newly added holdings or price changes to existing holdings, most commonly after market close.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HoldingsDefaultUpdateWebhook {
    /// `HOLDINGS`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `DEFAULT_UPDATE`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error: Option<Option<crate::models::PlaidError>>,
    /// The number of new holdings reported since the last time this webhook was fired.
    #[serde(rename = "new_holdings")]
    pub new_holdings: f32,
    /// The number of updated holdings reported since the last time this webhook was fired.
    #[serde(rename = "updated_holdings")]
    pub updated_holdings: f32,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl HoldingsDefaultUpdateWebhook {
    /// Fired when new or updated holdings have been detected on an investment account. The webhook typically fires in response to any newly added holdings or price changes to existing holdings, most commonly after market close.
    pub fn new(webhook_type: String, webhook_code: String, item_id: String, new_holdings: f32, updated_holdings: f32, environment: crate::models::WebhookEnvironmentValues) -> HoldingsDefaultUpdateWebhook {
        HoldingsDefaultUpdateWebhook {
            webhook_type,
            webhook_code,
            item_id,
            error: None,
            new_holdings,
            updated_holdings,
            environment,
        }
    }
}


