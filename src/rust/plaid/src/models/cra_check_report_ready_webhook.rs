/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraCheckReportReadyWebhook : Fired when the Check Report are ready to be retrieved. Once this webhook has fired, the report will be available to retrieve for 24 hours.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CraCheckReportReadyWebhook {
    /// `CHECK_REPORT`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `CHECK_REPORT_READY`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `user_id` corresponding to the user the webhook has fired for.
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// A list of `item_ids` that is included in the Check Report.
    #[serde(rename = "item_ids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub item_ids: Option<Option<Vec<String>>>,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl CraCheckReportReadyWebhook {
    /// Fired when the Check Report are ready to be retrieved. Once this webhook has fired, the report will be available to retrieve for 24 hours.
    pub fn new(webhook_type: String, webhook_code: String, user_id: String, environment: crate::models::WebhookEnvironmentValues) -> CraCheckReportReadyWebhook {
        CraCheckReportReadyWebhook {
            webhook_type,
            webhook_code,
            user_id,
            item_ids: None,
            environment,
        }
    }
}


