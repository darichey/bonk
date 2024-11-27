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

/// TransactionData : Information about the matched direct deposit transaction used to verify a user's payroll information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionData {
    /// The description of the transaction.
    #[serde(rename = "description")]
    pub description: String,
    /// The amount of the transaction.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// The date of the transaction, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (\"yyyy-mm-dd\").
    #[serde(rename = "date")]
    pub date: String,
    /// A unique identifier for the end user's account.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// A unique identifier for the transaction.
    #[serde(rename = "transaction_id")]
    pub transaction_id: String,
}

impl TransactionData {
    /// Information about the matched direct deposit transaction used to verify a user's payroll information.
    pub fn new(description: String, amount: f64, date: String, account_id: String, transaction_id: String) -> TransactionData {
        TransactionData {
            description,
            amount,
            date,
            account_id,
            transaction_id,
        }
    }
}

