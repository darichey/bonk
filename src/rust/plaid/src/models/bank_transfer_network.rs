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

/// BankTransferNetwork : The network or rails used for the transfer. Valid options are `ach`, `same-day-ach`, or `wire`.
/// The network or rails used for the transfer. Valid options are `ach`, `same-day-ach`, or `wire`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BankTransferNetwork {
    #[serde(rename = "ach")]
    Ach,
    #[serde(rename = "same-day-ach")]
    SameDayAch,
    #[serde(rename = "wire")]
    Wire,

}

impl std::fmt::Display for BankTransferNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Ach => write!(f, "ach"),
            Self::SameDayAch => write!(f, "same-day-ach"),
            Self::Wire => write!(f, "wire"),
        }
    }
}

impl Default for BankTransferNetwork {
    fn default() -> BankTransferNetwork {
        Self::Ach
    }
}

