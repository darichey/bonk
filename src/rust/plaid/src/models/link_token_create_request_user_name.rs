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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestUserName {
    /// A string with at least one non-whitespace character, with a max length of 100 characters.
    #[serde(rename = "given_name")]
    pub given_name: String,
    /// A string with at least one non-whitespace character, with a max length of 100 characters.
    #[serde(rename = "family_name")]
    pub family_name: String,
}

impl LinkTokenCreateRequestUserName {
    pub fn new(given_name: String, family_name: String) -> LinkTokenCreateRequestUserName {
        LinkTokenCreateRequestUserName {
            given_name,
            family_name,
        }
    }
}

