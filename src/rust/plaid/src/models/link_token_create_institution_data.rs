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

/// LinkTokenCreateInstitutionData : A map containing data used to highlight institutions in Link.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokenCreateInstitutionData {
    /// The routing number of the bank to highlight in Link. Note: in rare cases, a single routing number can be associated with multiple institutions, e.g. due to a brokerage using another institution to manage ACH on its sweep accounts. If this happens, the bank will not be highlighted in Link even if the routing number is provided.
    #[serde(rename = "routing_number", skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}

impl LinkTokenCreateInstitutionData {
    /// A map containing data used to highlight institutions in Link.
    pub fn new() -> LinkTokenCreateInstitutionData {
        LinkTokenCreateInstitutionData {
            routing_number: None,
        }
    }
}

