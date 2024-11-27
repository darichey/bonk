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

/// AssetReportRefreshRequest : AssetReportRefreshRequest defines the request schema for `/asset_report/refresh`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetReportRefreshRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The `asset_report_token` returned by the original call to `/asset_report/create`
    #[serde(rename = "asset_report_token")]
    pub asset_report_token: String,
    /// The maximum number of days of history to include in the Asset Report. Must be an integer. If not specified, the value from the original call to `/asset_report/create` will be used.
    #[serde(rename = "days_requested", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub days_requested: Option<Option<i32>>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<models::AssetReportRefreshRequestOptions>>,
}

impl AssetReportRefreshRequest {
    /// AssetReportRefreshRequest defines the request schema for `/asset_report/refresh`
    pub fn new(asset_report_token: String) -> AssetReportRefreshRequest {
        AssetReportRefreshRequest {
            client_id: None,
            secret: None,
            asset_report_token,
            days_requested: None,
            options: None,
        }
    }
}

