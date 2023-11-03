/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxBankIncomeFireWebhookRequest : SandboxBankIncomeFireWebhookRequest defines the request schema for `/sandbox/bank_income/fire_webhook`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SandboxBankIncomeFireWebhookRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The URL to which the webhook should be sent. If provided, this will override the URL set in the dashboard.
    #[serde(rename = "webhook_override", skip_serializing_if = "Option::is_none")]
    pub webhook_override: Option<String>,
    #[serde(rename = "webhook_code")]
    pub webhook_code: crate::models::SandboxBankIncomeWebhookFireRequestWebhookCode,
    #[serde(rename = "webhook_fields")]
    pub webhook_fields: Box<crate::models::SandboxBankIncomeWebhookFireRequestWebhookFields>,
}

impl SandboxBankIncomeFireWebhookRequest {
    /// SandboxBankIncomeFireWebhookRequest defines the request schema for `/sandbox/bank_income/fire_webhook`
    pub fn new(webhook_code: crate::models::SandboxBankIncomeWebhookFireRequestWebhookCode, webhook_fields: crate::models::SandboxBankIncomeWebhookFireRequestWebhookFields) -> SandboxBankIncomeFireWebhookRequest {
        SandboxBankIncomeFireWebhookRequest {
            client_id: None,
            secret: None,
            webhook_override: None,
            webhook_code,
            webhook_fields: Box::new(webhook_fields),
        }
    }
}


