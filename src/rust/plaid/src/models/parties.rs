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

/// Parties : A collection of objects that define specific parties to a deal. This includes the direct participating parties, such as borrower and seller and the indirect parties such as the credit report provider.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Parties {
    #[serde(rename = "PARTY")]
    pub party: Vec<models::Party>,
}

impl Parties {
    /// A collection of objects that define specific parties to a deal. This includes the direct participating parties, such as borrower and seller and the indirect parties such as the credit report provider.
    pub fn new(party: Vec<models::Party>) -> Parties {
        Parties {
            party,
        }
    }
}

