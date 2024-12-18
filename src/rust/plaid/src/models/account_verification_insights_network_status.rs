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

/// AccountVerificationInsightsNetworkStatus : Status information about the account and routing number in the Plaid network.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountVerificationInsightsNetworkStatus {
    /// Indicates whether we found at least one matching account for the ACH account and routing number.
    #[serde(rename = "has_numbers_match")]
    pub has_numbers_match: bool,
    /// Indicates if at least one matching account for the ACH account and routing number is already verified.
    #[serde(rename = "is_numbers_match_verified")]
    pub is_numbers_match_verified: bool,
}

impl AccountVerificationInsightsNetworkStatus {
    /// Status information about the account and routing number in the Plaid network.
    pub fn new(has_numbers_match: bool, is_numbers_match_verified: bool) -> AccountVerificationInsightsNetworkStatus {
        AccountVerificationInsightsNetworkStatus {
            has_numbers_match,
            is_numbers_match_verified,
        }
    }
}

