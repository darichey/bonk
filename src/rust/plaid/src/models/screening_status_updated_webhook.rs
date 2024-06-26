/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScreeningStatusUpdatedWebhook : Fired when an individual screening status has changed, which can occur manually via the dashboard or during ongoing monitoring.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScreeningStatusUpdatedWebhook {
    /// `SCREENING`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `STATUS_UPDATED`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The ID of the associated screening.
    #[serde(rename = "screening_id")]
    pub screening_id: String,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl ScreeningStatusUpdatedWebhook {
    /// Fired when an individual screening status has changed, which can occur manually via the dashboard or during ongoing monitoring.
    pub fn new(webhook_type: String, webhook_code: String, screening_id: String, environment: crate::models::WebhookEnvironmentValues) -> ScreeningStatusUpdatedWebhook {
        ScreeningStatusUpdatedWebhook {
            webhook_type,
            webhook_code,
            screening_id,
            environment,
        }
    }
}


