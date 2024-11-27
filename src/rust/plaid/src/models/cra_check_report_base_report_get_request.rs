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

/// CraCheckReportBaseReportGetRequest : BaseReportGetRequest defines the request schema for `/cra/check_report/base_report/get`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CraCheckReportBaseReportGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The user token associated with the User data is being requested for.
    #[serde(rename = "user_token", skip_serializing_if = "Option::is_none")]
    pub user_token: Option<String>,
    /// The third-party user token associated with the requested User data.
    #[serde(rename = "third_party_user_token", skip_serializing_if = "Option::is_none")]
    pub third_party_user_token: Option<String>,
    /// The item IDs to include in the Base Report. If not provided, all items associated with the user will be included.
    #[serde(rename = "item_ids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub item_ids: Option<Option<Vec<String>>>,
}

impl CraCheckReportBaseReportGetRequest {
    /// BaseReportGetRequest defines the request schema for `/cra/check_report/base_report/get`
    pub fn new() -> CraCheckReportBaseReportGetRequest {
        CraCheckReportBaseReportGetRequest {
            client_id: None,
            secret: None,
            user_token: None,
            third_party_user_token: None,
            item_ids: None,
        }
    }
}

