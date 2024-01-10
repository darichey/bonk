/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxBankIncomeWebhookFireRequestWebhookCode : The webhook codes this endpoint can be used to test

/// The webhook codes this endpoint can be used to test
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SandboxBankIncomeWebhookFireRequestWebhookCode {
    #[serde(rename = "BANK_INCOME_REFRESH_UPDATE")]
    Update,
    #[serde(rename = "BANK_INCOME_REFRESH_COMPLETE")]
    Complete,

}

impl ToString for SandboxBankIncomeWebhookFireRequestWebhookCode {
    fn to_string(&self) -> String {
        match self {
            Self::Update => String::from("BANK_INCOME_REFRESH_UPDATE"),
            Self::Complete => String::from("BANK_INCOME_REFRESH_COMPLETE"),
        }
    }
}

impl Default for SandboxBankIncomeWebhookFireRequestWebhookCode {
    fn default() -> SandboxBankIncomeWebhookFireRequestWebhookCode {
        Self::Update
    }
}



