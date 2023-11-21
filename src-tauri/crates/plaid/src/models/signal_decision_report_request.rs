/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SignalDecisionReportRequest : SignalDecisionReportRequest defines the request schema for `/signal/decision/report`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SignalDecisionReportRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Must be the same as the `client_transaction_id` supplied when calling `/signal/evaluate`
    #[serde(rename = "client_transaction_id")]
    pub client_transaction_id: String,
    /// `true` if the ACH transaction was initiated, `false` otherwise.  This field must be returned as a boolean. If formatted incorrectly, this will result in an [`INVALID_FIELD`](/docs/errors/invalid-request/#invalid_field) error.
    #[serde(rename = "initiated")]
    pub initiated: bool,
    /// The actual number of days (hold time) since the ACH debit transaction that you wait before making funds available to your customers. The holding time could affect the ACH return rate.  For example, use 0 if you make funds available to your customers instantly or the same day following the debit transaction, or 1 if you make funds available the next day following the debit initialization.
    #[serde(rename = "days_funds_on_hold", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub days_funds_on_hold: Option<Option<i32>>,
    #[serde(rename = "decision_outcome", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub decision_outcome: Option<Option<crate::models::SignalDecisionOutcome>>,
    #[serde(rename = "payment_method", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<Option<crate::models::SignalPaymentMethod>>,
    /// The amount (in USD) made available to your customers instantly following the debit transaction. It could be a partial amount of the requested transaction (example: 102.05).
    #[serde(rename = "amount_instantly_available", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub amount_instantly_available: Option<Option<f64>>,
}

impl SignalDecisionReportRequest {
    /// SignalDecisionReportRequest defines the request schema for `/signal/decision/report`
    pub fn new(client_transaction_id: String, initiated: bool) -> SignalDecisionReportRequest {
        SignalDecisionReportRequest {
            client_id: None,
            secret: None,
            client_transaction_id,
            initiated,
            days_funds_on_hold: None,
            decision_outcome: None,
            payment_method: None,
            amount_instantly_available: None,
        }
    }
}

