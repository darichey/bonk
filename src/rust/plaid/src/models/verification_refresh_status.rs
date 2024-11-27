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

/// VerificationRefreshStatus : The verification refresh status. One of the following:  `\"VERIFICATION_REFRESH_STATUS_USER_PRESENCE_REQUIRED\"` User presence is required to refresh an income verification. `\"VERIFICATION_REFRESH_SUCCESSFUL\"` The income verification refresh was successful. `\"VERIFICATION_REFRESH_NOT_FOUND\"` No new data was found after the income verification refresh.
/// The verification refresh status. One of the following:  `\"VERIFICATION_REFRESH_STATUS_USER_PRESENCE_REQUIRED\"` User presence is required to refresh an income verification. `\"VERIFICATION_REFRESH_SUCCESSFUL\"` The income verification refresh was successful. `\"VERIFICATION_REFRESH_NOT_FOUND\"` No new data was found after the income verification refresh.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VerificationRefreshStatus {
    #[serde(rename = "VERIFICATION_REFRESH_STATUS_USER_PRESENCE_REQUIRED")]
    StatusUserPresenceRequired,
    #[serde(rename = "VERIFICATION_REFRESH_SUCCESSFUL")]
    Successful,
    #[serde(rename = "VERIFICATION_REFRESH_NOT_FOUND")]
    NotFound,

}

impl std::fmt::Display for VerificationRefreshStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::StatusUserPresenceRequired => write!(f, "VERIFICATION_REFRESH_STATUS_USER_PRESENCE_REQUIRED"),
            Self::Successful => write!(f, "VERIFICATION_REFRESH_SUCCESSFUL"),
            Self::NotFound => write!(f, "VERIFICATION_REFRESH_NOT_FOUND"),
        }
    }
}

impl Default for VerificationRefreshStatus {
    fn default() -> VerificationRefreshStatus {
        Self::StatusUserPresenceRequired
    }
}

