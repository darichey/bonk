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

/// PoBoxStatus : Field describing whether the associated address is a post office box. Will be `yes` when a P.O. box is detected, `no` when Plaid confirmed the address is not a P.O. box, and `no_data` when Plaid was not able to determine if the address is a P.O. box.
/// Field describing whether the associated address is a post office box. Will be `yes` when a P.O. box is detected, `no` when Plaid confirmed the address is not a P.O. box, and `no_data` when Plaid was not able to determine if the address is a P.O. box.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PoBoxStatus {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "no_data")]
    NoData,

}

impl std::fmt::Display for PoBoxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Yes => write!(f, "yes"),
            Self::No => write!(f, "no"),
            Self::NoData => write!(f, "no_data"),
        }
    }
}

impl Default for PoBoxStatus {
    fn default() -> PoBoxStatus {
        Self::Yes
    }
}

