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

/// Credit1099Recipient : An object representing a recipient used in both 1099-K and 1099-MISC tax documents.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Credit1099Recipient {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<models::CreditPayStubAddress>,
    /// Name of recipient.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// Tax identification number of recipient.
    #[serde(rename = "tin", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tin: Option<Option<String>>,
    /// Account number number of recipient.
    #[serde(rename = "account_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_number: Option<Option<String>>,
    /// Checked if FACTA is a filing requirement.
    #[serde(rename = "facta_filing_requirement", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub facta_filing_requirement: Option<Option<String>>,
    /// Checked if 2nd TIN exists.
    #[serde(rename = "second_tin_exists", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub second_tin_exists: Option<Option<String>>,
}

impl Credit1099Recipient {
    /// An object representing a recipient used in both 1099-K and 1099-MISC tax documents.
    pub fn new() -> Credit1099Recipient {
        Credit1099Recipient {
            address: None,
            name: None,
            tin: None,
            account_number: None,
            facta_filing_requirement: None,
            second_tin_exists: None,
        }
    }
}

