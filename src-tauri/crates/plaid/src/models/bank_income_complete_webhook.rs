/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BankIncomeCompleteWebhook : Fired when a bank income report has finished generating or failed to generate, triggered by calling `/credit/bank_income/get` in CRA enabled client.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BankIncomeCompleteWebhook {
    /// `INCOME`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `BANK_INCOME_COMPLETE`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `user_id` corresponding to the user the webhook has fired for.
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "result")]
    pub result: crate::models::BankIncomeCompleteResult,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl BankIncomeCompleteWebhook {
    /// Fired when a bank income report has finished generating or failed to generate, triggered by calling `/credit/bank_income/get` in CRA enabled client.
    pub fn new(webhook_type: String, webhook_code: String, user_id: String, result: crate::models::BankIncomeCompleteResult, environment: crate::models::WebhookEnvironmentValues) -> BankIncomeCompleteWebhook {
        BankIncomeCompleteWebhook {
            webhook_type,
            webhook_code,
            user_id,
            result,
            environment,
        }
    }
}


