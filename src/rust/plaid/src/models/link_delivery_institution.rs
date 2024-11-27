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

/// LinkDeliveryInstitution : Information related to the financial institution.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkDeliveryInstitution {
    /// The full institution name, such as 'Wells Fargo'
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The Plaid institution identifier
    #[serde(rename = "institution_id", skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
}

impl LinkDeliveryInstitution {
    /// Information related to the financial institution.
    pub fn new() -> LinkDeliveryInstitution {
        LinkDeliveryInstitution {
            name: None,
            institution_id: None,
        }
    }
}

