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

/// BeaconAccountRiskEvaluateRequest : BeaconAccountRiskEvaluateRequest defines the request schema for `/v1/beacon/account_risk/risk/evaluate`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeaconAccountRiskEvaluateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The access token associated with the Item data is being requested for.
    #[serde(rename = "access_token", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<models::BeaconAccountRiskEvaluateRequestOptions>>,
    /// A unique ID that identifies the end user in your system. This ID is used to correlate requests by a user with multiple evaluations and/or multiple linked accounts. Personally identifiable information, such as an email address or phone number, should not be used in the client_user_id.
    #[serde(rename = "client_user_id", skip_serializing_if = "Option::is_none")]
    pub client_user_id: Option<String>,
    /// Unique identifier of what you are looking to evaluate (account add, information change, etc.) to allow us to tie the activity to the decisions and possible fraud outcome sent via our feedback endpoints. You can use your internal request ID or similar.
    #[serde(rename = "client_evaluation_id", skip_serializing_if = "Option::is_none")]
    pub client_evaluation_id: Option<String>,
    #[serde(rename = "evaluation_reason", skip_serializing_if = "Option::is_none")]
    pub evaluation_reason: Option<models::BeaconAccountRiskEvaluateEvaluationReason>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<Box<models::SignalDevice>>,
    /// The time the event for evaluation has occurred. Populate this field for backfilling data. If you don’t populate this field, we’ll use the timestamp at the time of receipt. Use ISO 8601 format (YYYY-MM-DDTHH:mm:ssZ).
    #[serde(rename = "evaluate_time", skip_serializing_if = "Option::is_none")]
    pub evaluate_time: Option<String>,
}

impl BeaconAccountRiskEvaluateRequest {
    /// BeaconAccountRiskEvaluateRequest defines the request schema for `/v1/beacon/account_risk/risk/evaluate`
    pub fn new() -> BeaconAccountRiskEvaluateRequest {
        BeaconAccountRiskEvaluateRequest {
            client_id: None,
            secret: None,
            access_token: None,
            options: None,
            client_user_id: None,
            client_evaluation_id: None,
            evaluation_reason: None,
            device: None,
            evaluate_time: None,
        }
    }
}

