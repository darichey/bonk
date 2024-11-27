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

/// LoanFilter : A filter to apply to `loan`-type accounts
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoanFilter {
    /// An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema). 
    #[serde(rename = "account_subtypes")]
    pub account_subtypes: Vec<models::LoanAccountSubtype>,
}

impl LoanFilter {
    /// A filter to apply to `loan`-type accounts
    pub fn new(account_subtypes: Vec<models::LoanAccountSubtype>) -> LoanFilter {
        LoanFilter {
            account_subtypes,
        }
    }
}

