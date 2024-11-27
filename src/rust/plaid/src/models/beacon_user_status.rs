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

/// BeaconUserStatus : A status of a Beacon User.  `rejected`: The Beacon User has been rejected for fraud. Users can be automatically or manually rejected.  `pending_review`: The Beacon User has been marked for review.  `cleared`: The Beacon User has been cleared of fraud.
/// A status of a Beacon User.  `rejected`: The Beacon User has been rejected for fraud. Users can be automatically or manually rejected.  `pending_review`: The Beacon User has been marked for review.  `cleared`: The Beacon User has been cleared of fraud.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BeaconUserStatus {
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "pending_review")]
    PendingReview,
    #[serde(rename = "cleared")]
    Cleared,

}

impl std::fmt::Display for BeaconUserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Rejected => write!(f, "rejected"),
            Self::PendingReview => write!(f, "pending_review"),
            Self::Cleared => write!(f, "cleared"),
        }
    }
}

impl Default for BeaconUserStatus {
    fn default() -> BeaconUserStatus {
        Self::Rejected
    }
}

