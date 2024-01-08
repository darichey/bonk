/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraBankIncomeCompleteWebhook : Fired when a bank income report has finished generating or failed to generate, triggered by calling `/cra/bank_income/get`.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CraBankIncomeCompleteWebhook {
    /// `CRA_INCOME`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `BANK_INCOME_COMPLETE`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `user_id` corresponding to the user the webhook has fired for.
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "result")]
    pub result: crate::models::CraBankIncomeCompleteResult,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl CraBankIncomeCompleteWebhook {
    /// Fired when a bank income report has finished generating or failed to generate, triggered by calling `/cra/bank_income/get`.
    pub fn new(webhook_type: String, webhook_code: String, user_id: String, result: crate::models::CraBankIncomeCompleteResult, environment: crate::models::WebhookEnvironmentValues) -> CraBankIncomeCompleteWebhook {
        CraBankIncomeCompleteWebhook {
            webhook_type,
            webhook_code,
            user_id,
            result,
            environment,
        }
    }
}


