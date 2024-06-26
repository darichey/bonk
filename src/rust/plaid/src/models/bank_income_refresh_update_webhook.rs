/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BankIncomeRefreshUpdateWebhook : Fired when a change to the user's income is detected. You should call `/credit/bank_income/refresh` to get updated income data for the user. To receive this webhook, subscribe in the [Dashboard](https://dashboard.plaid.com/developers/webhooks).



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BankIncomeRefreshUpdateWebhook {
    /// `INCOME`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `BANK_INCOME_REFRESH_UPDATE`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `user_id` corresponding to the user the webhook has fired for.
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl BankIncomeRefreshUpdateWebhook {
    /// Fired when a change to the user's income is detected. You should call `/credit/bank_income/refresh` to get updated income data for the user. To receive this webhook, subscribe in the [Dashboard](https://dashboard.plaid.com/developers/webhooks).
    pub fn new(webhook_type: String, webhook_code: String, user_id: String, environment: crate::models::WebhookEnvironmentValues) -> BankIncomeRefreshUpdateWebhook {
        BankIncomeRefreshUpdateWebhook {
            webhook_type,
            webhook_code,
            user_id,
            environment,
        }
    }
}


