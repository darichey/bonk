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

/// TransferEventListTransferType : The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into your origination account; a `credit` indicates a transfer of money out of your origination account.
/// The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into your origination account; a `credit` indicates a transfer of money out of your origination account.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferEventListTransferType {
    #[serde(rename = "debit")]
    Debit,
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "null")]
    Null,

}

impl std::fmt::Display for TransferEventListTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Debit => write!(f, "debit"),
            Self::Credit => write!(f, "credit"),
            Self::Null => write!(f, "null"),
        }
    }
}

impl Default for TransferEventListTransferType {
    fn default() -> TransferEventListTransferType {
        Self::Debit
    }
}

