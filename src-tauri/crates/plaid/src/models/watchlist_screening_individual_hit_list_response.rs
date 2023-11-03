/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WatchlistScreeningIndividualHitListResponse : Paginated list of individual watchlist screening hits



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualHitListResponse {
    /// List of individual watchlist screening hits
    #[serde(rename = "watchlist_screening_hits")]
    pub watchlist_screening_hits: Vec<crate::models::WatchlistScreeningHit>,
    /// An identifier that determines which page of results you receive.
    #[serde(rename = "next_cursor", deserialize_with = "Option::deserialize")]
    pub next_cursor: Option<String>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl WatchlistScreeningIndividualHitListResponse {
    /// Paginated list of individual watchlist screening hits
    pub fn new(watchlist_screening_hits: Vec<crate::models::WatchlistScreeningHit>, next_cursor: Option<String>, request_id: String) -> WatchlistScreeningIndividualHitListResponse {
        WatchlistScreeningIndividualHitListResponse {
            watchlist_screening_hits,
            next_cursor,
            request_id,
        }
    }
}


