/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AssetsErrorWebhook : Fired when Asset Report generation has failed. The resulting `error` will have an `error_type` of `ASSET_REPORT_ERROR`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetsErrorWebhook {
    /// `ASSETS`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `ERROR`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    #[serde(rename = "error", deserialize_with = "Option::deserialize")]
    pub error: Option<crate::models::PlaidError>,
    /// The ID associated with the Asset Report.
    #[serde(rename = "asset_report_id")]
    pub asset_report_id: String,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl AssetsErrorWebhook {
    /// Fired when Asset Report generation has failed. The resulting `error` will have an `error_type` of `ASSET_REPORT_ERROR`.
    pub fn new(webhook_type: String, webhook_code: String, error: Option<crate::models::PlaidError>, asset_report_id: String, environment: crate::models::WebhookEnvironmentValues) -> AssetsErrorWebhook {
        AssetsErrorWebhook {
            webhook_type,
            webhook_code,
            error,
            asset_report_id,
            environment,
        }
    }
}


