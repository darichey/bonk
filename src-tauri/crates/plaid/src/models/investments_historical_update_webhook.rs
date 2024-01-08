/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InvestmentsHistoricalUpdateWebhook : Fired after an asynchronous extraction on an investments account.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvestmentsHistoricalUpdateWebhook {
    /// `INVESTMENTS_TRANSACTIONS`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `HISTORICAL_UPDATE`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error: Option<Option<crate::models::PlaidError>>,
    /// The number of new transactions reported since the last time this webhook was fired.
    #[serde(rename = "new_investments_transactions")]
    pub new_investments_transactions: f32,
    /// The number of canceled transactions reported since the last time this webhook was fired.
    #[serde(rename = "canceled_investments_transactions")]
    pub canceled_investments_transactions: f32,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl InvestmentsHistoricalUpdateWebhook {
    /// Fired after an asynchronous extraction on an investments account.
    pub fn new(webhook_type: String, webhook_code: String, item_id: String, new_investments_transactions: f32, canceled_investments_transactions: f32, environment: crate::models::WebhookEnvironmentValues) -> InvestmentsHistoricalUpdateWebhook {
        InvestmentsHistoricalUpdateWebhook {
            webhook_type,
            webhook_code,
            item_id,
            error: None,
            new_investments_transactions,
            canceled_investments_transactions,
            environment,
        }
    }
}


