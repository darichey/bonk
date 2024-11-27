/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SignalReturnReportRequest : SignalReturnReportRequest defines the request schema for `/signal/return/report`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignalReturnReportRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Must be the same as the `client_transaction_id` supplied when calling `/signal/evaluate` or `/accounts/balance/get`.
    #[serde(rename = "client_transaction_id")]
    pub client_transaction_id: String,
    /// Must be a valid ACH return code (e.g. \"R01\")  If formatted incorrectly, this will result in an [`INVALID_FIELD`](/docs/errors/invalid-request/#invalid_field) error.
    #[serde(rename = "return_code")]
    pub return_code: String,
    /// Date and time when you receive the returns from your payment processors, in ISO 8601 format (`YYYY-MM-DDTHH:mm:ssZ`).
    #[serde(rename = "returned_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<Option<String>>,
}

impl SignalReturnReportRequest {
    /// SignalReturnReportRequest defines the request schema for `/signal/return/report`
    pub fn new(client_transaction_id: String, return_code: String) -> SignalReturnReportRequest {
        SignalReturnReportRequest {
            client_id: None,
            secret: None,
            client_transaction_id,
            return_code,
            returned_at: None,
        }
    }
}

