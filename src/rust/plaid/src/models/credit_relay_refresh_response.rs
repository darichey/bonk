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

/// CreditRelayRefreshResponse : CreditRelayRefreshResponse defines the response schema for `/credit/relay/refresh`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditRelayRefreshResponse {
    #[serde(rename = "relay_token")]
    pub relay_token: String,
    /// A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    #[serde(rename = "asset_report_id", skip_serializing_if = "Option::is_none")]
    pub asset_report_id: Option<String>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl CreditRelayRefreshResponse {
    /// CreditRelayRefreshResponse defines the response schema for `/credit/relay/refresh`
    pub fn new(relay_token: String, request_id: String) -> CreditRelayRefreshResponse {
        CreditRelayRefreshResponse {
            relay_token,
            asset_report_id: None,
            request_id,
        }
    }
}

