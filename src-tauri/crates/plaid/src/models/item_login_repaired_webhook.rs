/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemLoginRepairedWebhook : Fired when an Item has exited the `ITEM_LOGIN_REQUIRED` state without the user having gone through the update mode flow in your app (this can happen if the user completed the update mode in a different app). If you have messaging that tells the user to complete the update mode flow, you should silence this messaging upon receiving the `LOGIN_REPAIRED` webhook.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemLoginRepairedWebhook {
    /// `ITEM`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `LOGIN_REPAIRED`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl ItemLoginRepairedWebhook {
    /// Fired when an Item has exited the `ITEM_LOGIN_REQUIRED` state without the user having gone through the update mode flow in your app (this can happen if the user completed the update mode in a different app). If you have messaging that tells the user to complete the update mode flow, you should silence this messaging upon receiving the `LOGIN_REPAIRED` webhook.
    pub fn new(webhook_type: String, webhook_code: String, item_id: String, environment: crate::models::WebhookEnvironmentValues) -> ItemLoginRepairedWebhook {
        ItemLoginRepairedWebhook {
            webhook_type,
            webhook_code,
            item_id,
            environment,
        }
    }
}


