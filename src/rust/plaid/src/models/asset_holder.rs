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

/// AssetHolder : Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetHolder {
    #[serde(rename = "NAME")]
    pub name: models::AssetHolderName,
}

impl AssetHolder {
    /// Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    pub fn new(name: models::AssetHolderName) -> AssetHolder {
        AssetHolder {
            name,
        }
    }
}

