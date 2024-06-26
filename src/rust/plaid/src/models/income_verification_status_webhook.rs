/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IncomeVerificationStatusWebhook : Fired when the status of an income verification instance has changed. It will typically take several minutes for this webhook to fire after the end user has uploaded their documents in the Document Income flow.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncomeVerificationStatusWebhook {
    /// `\"INCOME\"`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `INCOME_VERIFICATION`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The Item ID associated with the verification.
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The Plaid `user_id` of the User associated with this webhook, warning, or error.
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// `VERIFICATION_STATUS_PROCESSING_COMPLETE`:  The income verification processing has completed. This indicates that the documents have been parsed successfully or that the documents were not parsable. If the user uploaded multiple documents, this webhook will fire when all documents have finished processing. Call the `/credit/payroll_income/get` endpoint and check the document metadata to see which documents were successfully parsed.  `VERIFICATION_STATUS_PROCESSING_FAILED`: An unexpected internal error occurred when attempting to process the verification documentation.  `VERIFICATION_STATUS_PENDING_APPROVAL`: (deprecated) The income verification has been sent to the user for review.
    #[serde(rename = "verification_status")]
    pub verification_status: String,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl IncomeVerificationStatusWebhook {
    /// Fired when the status of an income verification instance has changed. It will typically take several minutes for this webhook to fire after the end user has uploaded their documents in the Document Income flow.
    pub fn new(webhook_type: String, webhook_code: String, item_id: String, verification_status: String, environment: crate::models::WebhookEnvironmentValues) -> IncomeVerificationStatusWebhook {
        IncomeVerificationStatusWebhook {
            webhook_type,
            webhook_code,
            item_id,
            user_id: None,
            verification_status,
            environment,
        }
    }
}


