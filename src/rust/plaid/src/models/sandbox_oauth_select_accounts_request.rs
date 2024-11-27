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

/// SandboxOauthSelectAccountsRequest : Defines the request schema for `sandbox/oauth/select_accounts`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SandboxOauthSelectAccountsRequest {
    #[serde(rename = "oauth_state_id")]
    pub oauth_state_id: String,
    #[serde(rename = "accounts")]
    pub accounts: Vec<String>,
}

impl SandboxOauthSelectAccountsRequest {
    /// Defines the request schema for `sandbox/oauth/select_accounts`
    pub fn new(oauth_state_id: String, accounts: Vec<String>) -> SandboxOauthSelectAccountsRequest {
        SandboxOauthSelectAccountsRequest {
            oauth_state_id,
            accounts,
        }
    }
}

