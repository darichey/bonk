/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// VerificationExpiredWebhook : Fired when an Item was not verified via automated micro-deposits after seven days since the automated micro-deposit was made.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VerificationExpiredWebhook {
    /// `AUTH`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `VERIFICATION_EXPIRED`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The `account_id` of the account associated with the webhook
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl VerificationExpiredWebhook {
    /// Fired when an Item was not verified via automated micro-deposits after seven days since the automated micro-deposit was made.
    pub fn new(webhook_type: String, webhook_code: String, item_id: String, account_id: String, environment: crate::models::WebhookEnvironmentValues) -> VerificationExpiredWebhook {
        VerificationExpiredWebhook {
            webhook_type,
            webhook_code,
            item_id,
            account_id,
            environment,
        }
    }
}

