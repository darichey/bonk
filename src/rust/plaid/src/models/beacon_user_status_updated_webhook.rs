/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BeaconUserStatusUpdatedWebhook : Fired when a Beacon User status has changed, which can occur manually via the dashboard or when information is reported to the Beacon network.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeaconUserStatusUpdatedWebhook {
    /// `BEACON`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `USER_STATUS_UPDATED`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The ID of the associated Beacon user.
    #[serde(rename = "beacon_user_id")]
    pub beacon_user_id: String,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl BeaconUserStatusUpdatedWebhook {
    /// Fired when a Beacon User status has changed, which can occur manually via the dashboard or when information is reported to the Beacon network.
    pub fn new(webhook_type: String, webhook_code: String, beacon_user_id: String, environment: crate::models::WebhookEnvironmentValues) -> BeaconUserStatusUpdatedWebhook {
        BeaconUserStatusUpdatedWebhook {
            webhook_type,
            webhook_code,
            beacon_user_id,
            environment,
        }
    }
}


