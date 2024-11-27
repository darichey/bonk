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

/// WatchlistScreeningDocument : An official document, usually issued by a governing body or institution, with an associated identifier.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchlistScreeningDocument {
    #[serde(rename = "type")]
    pub r#type: models::WatchlistScreeningDocumentType,
    /// The numeric or alphanumeric identifier associated with this document. Must be between 4 and 32 characters long, and cannot have leading or trailing spaces.
    #[serde(rename = "number")]
    pub number: String,
}

impl WatchlistScreeningDocument {
    /// An official document, usually issued by a governing body or institution, with an associated identifier.
    pub fn new(r#type: models::WatchlistScreeningDocumentType, number: String) -> WatchlistScreeningDocument {
        WatchlistScreeningDocument {
            r#type,
            number,
        }
    }
}

