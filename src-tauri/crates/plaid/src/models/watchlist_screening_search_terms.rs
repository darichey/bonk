/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WatchlistScreeningSearchTerms : Search terms for creating an individual watchlist screening



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchlistScreeningSearchTerms {
    /// ID of the associated program.
    #[serde(rename = "watchlist_program_id")]
    pub watchlist_program_id: String,
    /// The legal name of the individual being screened.
    #[serde(rename = "legal_name")]
    pub legal_name: String,
    /// A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(rename = "date_of_birth", deserialize_with = "Option::deserialize")]
    pub date_of_birth: Option<String>,
    /// The numeric or alphanumeric identifier associated with this document.
    #[serde(rename = "document_number", deserialize_with = "Option::deserialize")]
    pub document_number: Option<String>,
    /// Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    #[serde(rename = "country", deserialize_with = "Option::deserialize")]
    pub country: Option<String>,
    /// The current version of the search terms. Starts at `1` and increments with each edit to `search_terms`.
    #[serde(rename = "version")]
    pub version: i32,
}

impl WatchlistScreeningSearchTerms {
    /// Search terms for creating an individual watchlist screening
    pub fn new(watchlist_program_id: String, legal_name: String, date_of_birth: Option<String>, document_number: Option<String>, country: Option<String>, version: i32) -> WatchlistScreeningSearchTerms {
        WatchlistScreeningSearchTerms {
            watchlist_program_id,
            legal_name,
            date_of_birth,
            document_number,
            country,
            version,
        }
    }
}


