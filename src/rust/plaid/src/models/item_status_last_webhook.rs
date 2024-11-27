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

/// ItemStatusLastWebhook : Information about the last webhook fired for the Item.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemStatusLastWebhook {
    /// [ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of when the webhook was fired. 
    #[serde(rename = "sent_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<Option<String>>,
    /// The last webhook code sent.
    #[serde(rename = "code_sent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub code_sent: Option<Option<String>>,
}

impl ItemStatusLastWebhook {
    /// Information about the last webhook fired for the Item.
    pub fn new() -> ItemStatusLastWebhook {
        ItemStatusLastWebhook {
            sent_at: None,
            code_sent: None,
        }
    }
}

