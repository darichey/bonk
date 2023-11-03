/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LiabilitiesDefaultUpdateWebhook : The webhook of type `LIABILITIES` and code `DEFAULT_UPDATE` will be fired when new or updated liabilities have been detected on a liabilities item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LiabilitiesDefaultUpdateWebhook {
    /// `LIABILITIES`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `DEFAULT_UPDATE`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[serde(rename = "error", deserialize_with = "Option::deserialize")]
    pub error: Option<crate::models::PlaidError>,
    /// An array of `account_id`'s for accounts that contain new liabilities.'
    #[serde(rename = "account_ids_with_new_liabilities")]
    pub account_ids_with_new_liabilities: Vec<String>,
    /// An object with keys of `account_id`'s that are mapped to their respective liabilities fields that changed.  Example: `{ \"XMBvvyMGQ1UoLbKByoMqH3nXMj84ALSdE5B58\": [\"past_amount_due\"] }` 
    #[serde(rename = "account_ids_with_updated_liabilities")]
    pub account_ids_with_updated_liabilities: ::std::collections::HashMap<String, Vec<String>>,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl LiabilitiesDefaultUpdateWebhook {
    /// The webhook of type `LIABILITIES` and code `DEFAULT_UPDATE` will be fired when new or updated liabilities have been detected on a liabilities item.
    pub fn new(webhook_type: String, webhook_code: String, item_id: String, error: Option<crate::models::PlaidError>, account_ids_with_new_liabilities: Vec<String>, account_ids_with_updated_liabilities: ::std::collections::HashMap<String, Vec<String>>, environment: crate::models::WebhookEnvironmentValues) -> LiabilitiesDefaultUpdateWebhook {
        LiabilitiesDefaultUpdateWebhook {
            webhook_type,
            webhook_code,
            item_id,
            error,
            account_ids_with_new_liabilities,
            account_ids_with_updated_liabilities,
            environment,
        }
    }
}


