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

/// CreditSessionDocumentIncomeResult : The details of a document income verification in Link
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditSessionDocumentIncomeResult {
    /// The number of paystubs uploaded by the user.
    #[serde(rename = "num_paystubs_uploaded")]
    pub num_paystubs_uploaded: i32,
    /// The number of w2s uploaded by the user.
    #[serde(rename = "num_w2s_uploaded")]
    pub num_w2s_uploaded: i32,
    /// The number of bank statements uploaded by the user.
    #[serde(rename = "num_bank_statements_uploaded")]
    pub num_bank_statements_uploaded: i32,
    /// The number of 1099s uploaded by the user
    #[serde(rename = "num_1099s_uploaded")]
    pub num_1099s_uploaded: i32,
}

impl CreditSessionDocumentIncomeResult {
    /// The details of a document income verification in Link
    pub fn new(num_paystubs_uploaded: i32, num_w2s_uploaded: i32, num_bank_statements_uploaded: i32, num_1099s_uploaded: i32) -> CreditSessionDocumentIncomeResult {
        CreditSessionDocumentIncomeResult {
            num_paystubs_uploaded,
            num_w2s_uploaded,
            num_bank_statements_uploaded,
            num_1099s_uploaded,
        }
    }
}

