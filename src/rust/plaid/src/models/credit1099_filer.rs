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

/// Credit1099Filer : An object representing a filer used by 1099-K tax documents.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Credit1099Filer {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<models::CreditPayStubAddress>,
    /// Name of filer.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// Tax identification number of filer.
    #[serde(rename = "tin", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tin: Option<Option<String>>,
    /// One of the following values will be provided: Payment Settlement Entity (PSE), Electronic Payment Facilitator (EPF), Other Third Party
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<String>>,
}

impl Credit1099Filer {
    /// An object representing a filer used by 1099-K tax documents.
    pub fn new() -> Credit1099Filer {
        Credit1099Filer {
            address: None,
            name: None,
            tin: None,
            r#type: None,
        }
    }
}

