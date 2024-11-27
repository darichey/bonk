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

/// WalletTransactionCounterpartyInternational : International Bank Account Number for a Wallet Transaction
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WalletTransactionCounterpartyInternational {
    /// International Bank Account Number (IBAN).
    #[serde(rename = "iban", skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
}

impl WalletTransactionCounterpartyInternational {
    /// International Bank Account Number for a Wallet Transaction
    pub fn new() -> WalletTransactionCounterpartyInternational {
        WalletTransactionCounterpartyInternational {
            iban: None,
        }
    }
}

