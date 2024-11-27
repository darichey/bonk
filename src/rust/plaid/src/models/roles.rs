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

/// Roles : Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Roles {
    #[serde(rename = "ROLE")]
    pub role: models::Role,
}

impl Roles {
    /// Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    pub fn new(role: models::Role) -> Roles {
        Roles {
            role,
        }
    }
}

