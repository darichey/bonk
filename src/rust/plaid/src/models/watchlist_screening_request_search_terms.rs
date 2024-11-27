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

/// WatchlistScreeningRequestSearchTerms : Search inputs for creating a watchlist screening
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchlistScreeningRequestSearchTerms {
    /// ID of the associated program.
    #[serde(rename = "watchlist_program_id")]
    pub watchlist_program_id: String,
    /// The legal name of the individual being screened. Must have at least one alphabetical character, have a maximum length of 100 characters, and not include leading or trailing spaces.
    #[serde(rename = "legal_name")]
    pub legal_name: String,
    /// A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(rename = "date_of_birth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// The numeric or alphanumeric identifier associated with this document. Must be between 4 and 32 characters long, and cannot have leading or trailing spaces.
    #[serde(rename = "document_number", skip_serializing_if = "Option::is_none")]
    pub document_number: Option<String>,
    /// Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl WatchlistScreeningRequestSearchTerms {
    /// Search inputs for creating a watchlist screening
    pub fn new(watchlist_program_id: String, legal_name: String) -> WatchlistScreeningRequestSearchTerms {
        WatchlistScreeningRequestSearchTerms {
            watchlist_program_id,
            legal_name,
            date_of_birth: None,
            document_number: None,
            country: None,
        }
    }
}

