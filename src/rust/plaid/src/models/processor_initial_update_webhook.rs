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

/// ProcessorInitialUpdateWebhook : This webhook is only sent to [Plaid processor partners](https://plaid.com/docs/auth/partnerships/).  Fired when an Item's initial transaction pull is completed. Once this webhook has been fired, transaction data for the most recent 30 days can be fetched for the Item. This webhook will also be fired if account selections for the Item are updated, with `new_transactions` set to the number of net new transactions pulled after the account selection update.  This webhook is intended for use with `/processor/transactions/get`; if you are using the newer `/processor/transactions/sync` endpoint, this webhook will still be fired to maintain backwards compatibility, but it is recommended to listen for and respond to the `SYNC_UPDATES_AVAILABLE` webhook instead.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessorInitialUpdateWebhook {
    /// `TRANSACTIONS`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `INITIAL_UPDATE`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The error code associated with the webhook.
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error: Option<Option<String>>,
    /// The number of new transactions available.
    #[serde(rename = "new_transactions")]
    pub new_transactions: f64,
    /// The ID of the account.
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "environment")]
    pub environment: models::WebhookEnvironmentValues,
}

impl ProcessorInitialUpdateWebhook {
    /// This webhook is only sent to [Plaid processor partners](https://plaid.com/docs/auth/partnerships/).  Fired when an Item's initial transaction pull is completed. Once this webhook has been fired, transaction data for the most recent 30 days can be fetched for the Item. This webhook will also be fired if account selections for the Item are updated, with `new_transactions` set to the number of net new transactions pulled after the account selection update.  This webhook is intended for use with `/processor/transactions/get`; if you are using the newer `/processor/transactions/sync` endpoint, this webhook will still be fired to maintain backwards compatibility, but it is recommended to listen for and respond to the `SYNC_UPDATES_AVAILABLE` webhook instead.
    pub fn new(webhook_type: String, webhook_code: String, new_transactions: f64, account_id: String, environment: models::WebhookEnvironmentValues) -> ProcessorInitialUpdateWebhook {
        ProcessorInitialUpdateWebhook {
            webhook_type,
            webhook_code,
            error: None,
            new_transactions,
            account_id,
            environment,
        }
    }
}

