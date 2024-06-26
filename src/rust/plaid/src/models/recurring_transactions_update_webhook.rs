/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RecurringTransactionsUpdateWebhook : Fired when recurring transactions data is updated. This includes when a new recurring stream is detected or when a new transaction is added to an existing recurring stream. The `RECURRING_TRANSACTIONS_UPDATE` webhook will also fire when one or more attributes of the recurring stream changes, which is usually a result of the addition, update, or removal of transactions to the stream.  After receipt of this webhook, the updated data can be fetched from `/transactions/recurring/get`.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecurringTransactionsUpdateWebhook {
    /// `TRANSACTIONS`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `RECURRING_TRANSACTIONS_UPDATE`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// A list of `account_ids` for accounts that have new or updated recurring transactions data.
    #[serde(rename = "account_ids")]
    pub account_ids: Vec<String>,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl RecurringTransactionsUpdateWebhook {
    /// Fired when recurring transactions data is updated. This includes when a new recurring stream is detected or when a new transaction is added to an existing recurring stream. The `RECURRING_TRANSACTIONS_UPDATE` webhook will also fire when one or more attributes of the recurring stream changes, which is usually a result of the addition, update, or removal of transactions to the stream.  After receipt of this webhook, the updated data can be fetched from `/transactions/recurring/get`.
    pub fn new(webhook_type: String, webhook_code: String, item_id: String, account_ids: Vec<String>, environment: crate::models::WebhookEnvironmentValues) -> RecurringTransactionsUpdateWebhook {
        RecurringTransactionsUpdateWebhook {
            webhook_type,
            webhook_code,
            item_id,
            account_ids,
            environment,
        }
    }
}


