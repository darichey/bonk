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

/// CraBankIncomeCreateRequest : CraBankIncomeCreateRequest defines the request schema for `/cra/bank_income/create`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CraBankIncomeCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The user token associated with the User data is being requested for.
    #[serde(rename = "user_token", skip_serializing_if = "Option::is_none")]
    pub user_token: Option<String>,
    /// The destination URL to which webhooks will be sent 
    #[serde(rename = "webhook", skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
    /// The number of days of data to request
    #[serde(rename = "days_requested", skip_serializing_if = "Option::is_none")]
    pub days_requested: Option<i32>,
    #[serde(rename = "consumer_report_permissible_purpose", skip_serializing_if = "Option::is_none")]
    pub consumer_report_permissible_purpose: Option<models::ConsumerReportPermissiblePurpose>,
}

impl CraBankIncomeCreateRequest {
    /// CraBankIncomeCreateRequest defines the request schema for `/cra/bank_income/create`.
    pub fn new() -> CraBankIncomeCreateRequest {
        CraBankIncomeCreateRequest {
            client_id: None,
            secret: None,
            user_token: None,
            webhook: None,
            days_requested: None,
            consumer_report_permissible_purpose: None,
        }
    }
}

