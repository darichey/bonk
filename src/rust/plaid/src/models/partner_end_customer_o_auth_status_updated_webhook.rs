/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PartnerEndCustomerOAuthStatusUpdatedWebhook : The webhook of type `PARTNER` and code `END_CUSTOMER_OAUTH_STATUS_UPDATED` will be fired when a partner's end customer has an update on their OAuth registration status with an institution.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PartnerEndCustomerOAuthStatusUpdatedWebhook {
    /// `PARTNER`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `END_CUSTOMER_OAUTH_STATUS_UPDATED`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The client ID of the end customer
    #[serde(rename = "end_customer_client_id")]
    pub end_customer_client_id: String,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
    /// The institution ID
    #[serde(rename = "institution_id")]
    pub institution_id: String,
    /// The institution name
    #[serde(rename = "institution_name")]
    pub institution_name: String,
    #[serde(rename = "status")]
    pub status: crate::models::PartnerEndCustomerOAuthStatusUpdatedValues,
}

impl PartnerEndCustomerOAuthStatusUpdatedWebhook {
    /// The webhook of type `PARTNER` and code `END_CUSTOMER_OAUTH_STATUS_UPDATED` will be fired when a partner's end customer has an update on their OAuth registration status with an institution.
    pub fn new(webhook_type: String, webhook_code: String, end_customer_client_id: String, environment: crate::models::WebhookEnvironmentValues, institution_id: String, institution_name: String, status: crate::models::PartnerEndCustomerOAuthStatusUpdatedValues) -> PartnerEndCustomerOAuthStatusUpdatedWebhook {
        PartnerEndCustomerOAuthStatusUpdatedWebhook {
            webhook_type,
            webhook_code,
            end_customer_client_id,
            environment,
            institution_id,
            institution_name,
            status,
        }
    }
}


