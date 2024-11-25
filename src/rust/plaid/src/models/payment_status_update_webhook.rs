/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentStatusUpdateWebhook : Fired when the status of a payment has changed.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentStatusUpdateWebhook {
    /// `PAYMENT_INITIATION`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `PAYMENT_STATUS_UPDATE`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `payment_id` for the payment being updated
    #[serde(rename = "payment_id")]
    pub payment_id: String,
    /// The transaction ID that this payment is associated with, if any. This is present only when a payment was initiated using virtual accounts.
    #[serde(rename = "transaction_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<Option<String>>,
    #[serde(rename = "new_payment_status")]
    pub new_payment_status: crate::models::PaymentInitiationPaymentStatus,
    #[serde(rename = "old_payment_status")]
    pub old_payment_status: crate::models::PaymentInitiationPaymentStatus,
    /// The original value of the reference when creating the payment.
    #[serde(rename = "original_reference", deserialize_with = "Option::deserialize")]
    pub original_reference: Option<String>,
    /// The value of the reference sent to the bank after adjustment to pass bank validation rules.
    #[serde(rename = "adjusted_reference", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub adjusted_reference: Option<Option<String>>,
    /// The original value of the `start_date` provided during the creation of a standing order. If the payment is not a standing order, this field will be `null`.
    #[serde(rename = "original_start_date", deserialize_with = "Option::deserialize")]
    pub original_start_date: Option<String>,
    /// The start date sent to the bank after adjusting for holidays or weekends.  Will be provided in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). If the start date did not require adjustment, or if the payment is not a standing order, this field will be `null`.
    #[serde(rename = "adjusted_start_date", deserialize_with = "Option::deserialize")]
    pub adjusted_start_date: Option<String>,
    /// The timestamp of the update, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `\"2017-09-14T14:42:19.350Z\"`
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error: Option<Option<crate::models::PlaidError>>,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl PaymentStatusUpdateWebhook {
    /// Fired when the status of a payment has changed.
    pub fn new(webhook_type: String, webhook_code: String, payment_id: String, new_payment_status: crate::models::PaymentInitiationPaymentStatus, old_payment_status: crate::models::PaymentInitiationPaymentStatus, original_reference: Option<String>, original_start_date: Option<String>, adjusted_start_date: Option<String>, timestamp: String, environment: crate::models::WebhookEnvironmentValues) -> PaymentStatusUpdateWebhook {
        PaymentStatusUpdateWebhook {
            webhook_type,
            webhook_code,
            payment_id,
            transaction_id: None,
            new_payment_status,
            old_payment_status,
            original_reference,
            adjusted_reference: None,
            original_start_date,
            adjusted_start_date,
            timestamp,
            error: None,
            environment,
        }
    }
}


