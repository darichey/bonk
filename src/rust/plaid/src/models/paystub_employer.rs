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

/// PaystubEmployer : Information about the employer on the paystub
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaystubEmployer {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<models::PaystubAddress>,
    /// The name of the employer on the paystub.
    #[serde(rename = "name", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
}

impl PaystubEmployer {
    /// Information about the employer on the paystub
    pub fn new(name: Option<String>) -> PaystubEmployer {
        PaystubEmployer {
            address: None,
            name,
        }
    }
}

