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

/// CreditRelayPdfGetRequest : CreditRelayPDFGetRequest defines the request schema for `/credit/relay/pdf/get`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditRelayPdfGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The `relay_token` granting access to the report you would like to get.
    #[serde(rename = "relay_token")]
    pub relay_token: String,
    #[serde(rename = "report_type")]
    pub report_type: models::ReportType,
}

impl CreditRelayPdfGetRequest {
    /// CreditRelayPDFGetRequest defines the request schema for `/credit/relay/pdf/get`
    pub fn new(relay_token: String, report_type: models::ReportType) -> CreditRelayPdfGetRequest {
        CreditRelayPdfGetRequest {
            client_id: None,
            secret: None,
            relay_token,
            report_type,
        }
    }
}

