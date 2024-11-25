/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxBankIncomeWebhookFireRequestWebhookFields : Optional fields which will be populated in the simulated webhook



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SandboxBankIncomeWebhookFireRequestWebhookFields {
    /// The user id to be returned in INCOME webhooks
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "bank_income_refresh_complete_result", skip_serializing_if = "Option::is_none")]
    pub bank_income_refresh_complete_result: Option<crate::models::BankIncomeRefreshCompleteResult>,
}

impl SandboxBankIncomeWebhookFireRequestWebhookFields {
    /// Optional fields which will be populated in the simulated webhook
    pub fn new(user_id: String) -> SandboxBankIncomeWebhookFireRequestWebhookFields {
        SandboxBankIncomeWebhookFireRequestWebhookFields {
            user_id,
            bank_income_refresh_complete_result: None,
        }
    }
}


