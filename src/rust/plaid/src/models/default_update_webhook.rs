/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DefaultUpdateWebhook : Fired when new transaction data is available for an Item. Plaid will typically check for new transaction data several times a day.  This webhook is intended for use with `/transactions/get`; if you are using the newer `/transactions/sync` endpoint, this webhook will still be fired to maintain backwards compatibility, but it is recommended to listen for and respond to the `SYNC_UPDATES_AVAILABLE` webhook instead. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DefaultUpdateWebhook {
    /// `TRANSACTIONS`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `DEFAULT_UPDATE`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error: Option<Option<crate::models::PlaidError>>,
    /// The number of new transactions detected since the last time this webhook was fired.
    #[serde(rename = "new_transactions")]
    pub new_transactions: f32,
    /// The `item_id` of the Item the webhook relates to.
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl DefaultUpdateWebhook {
    /// Fired when new transaction data is available for an Item. Plaid will typically check for new transaction data several times a day.  This webhook is intended for use with `/transactions/get`; if you are using the newer `/transactions/sync` endpoint, this webhook will still be fired to maintain backwards compatibility, but it is recommended to listen for and respond to the `SYNC_UPDATES_AVAILABLE` webhook instead. 
    pub fn new(webhook_type: String, webhook_code: String, new_transactions: f32, item_id: String, environment: crate::models::WebhookEnvironmentValues) -> DefaultUpdateWebhook {
        DefaultUpdateWebhook {
            webhook_type,
            webhook_code,
            error: None,
            new_transactions,
            item_id,
            environment,
        }
    }
}


