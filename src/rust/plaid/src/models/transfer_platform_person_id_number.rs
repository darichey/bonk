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

/// TransferPlatformPersonIdNumber : ID number of the person
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferPlatformPersonIdNumber {
    /// Value of the person's ID Number. Alpha-numeric, with all formatting characters stripped.
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "type")]
    pub r#type: models::IdNumberType,
}

impl TransferPlatformPersonIdNumber {
    /// ID number of the person
    pub fn new(value: String, r#type: models::IdNumberType) -> TransferPlatformPersonIdNumber {
        TransferPlatformPersonIdNumber {
            value,
            r#type,
        }
    }
}

