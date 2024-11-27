/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ProcessorRecurringTransactionsUpdateWebhook : This webhook is only sent to [Plaid processor partners](https://plaid.com/docs/auth/partnerships/).  Fired when recurring transactions data is updated. This includes when a new recurring stream is detected or when a new transaction is added to an existing recurring stream. The `RECURRING_TRANSACTIONS_UPDATE` webhook will also fire when one or more attributes of the recurring stream changes, which is usually a result of the addition, update, or removal of transactions to the stream.  After receipt of this webhook, the updated data can be fetched from `/processor/transactions/recurring/get`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessorRecurringTransactionsUpdateWebhook {
    /// `TRANSACTIONS`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `RECURRING_TRANSACTIONS_UPDATE`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The ID of the account.
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "environment")]
    pub environment: models::WebhookEnvironmentValues,
}

impl ProcessorRecurringTransactionsUpdateWebhook {
    /// This webhook is only sent to [Plaid processor partners](https://plaid.com/docs/auth/partnerships/).  Fired when recurring transactions data is updated. This includes when a new recurring stream is detected or when a new transaction is added to an existing recurring stream. The `RECURRING_TRANSACTIONS_UPDATE` webhook will also fire when one or more attributes of the recurring stream changes, which is usually a result of the addition, update, or removal of transactions to the stream.  After receipt of this webhook, the updated data can be fetched from `/processor/transactions/recurring/get`.
    pub fn new(webhook_type: String, webhook_code: String, account_id: String, environment: models::WebhookEnvironmentValues) -> ProcessorRecurringTransactionsUpdateWebhook {
        ProcessorRecurringTransactionsUpdateWebhook {
            webhook_type,
            webhook_code,
            account_id,
            environment,
        }
    }
}

