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

/// DashboardUserListResponse : Paginated list of dashboard users
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardUserListResponse {
    /// List of dashboard users
    #[serde(rename = "dashboard_users")]
    pub dashboard_users: Vec<models::DashboardUser>,
    /// An identifier that determines which page of results you receive.
    #[serde(rename = "next_cursor", deserialize_with = "Option::deserialize")]
    pub next_cursor: Option<String>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl DashboardUserListResponse {
    /// Paginated list of dashboard users
    pub fn new(dashboard_users: Vec<models::DashboardUser>, next_cursor: Option<String>, request_id: String) -> DashboardUserListResponse {
        DashboardUserListResponse {
            dashboard_users,
            next_cursor,
            request_id,
        }
    }
}

