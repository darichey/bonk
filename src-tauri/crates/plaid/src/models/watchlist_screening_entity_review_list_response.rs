/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WatchlistScreeningEntityReviewListResponse : Paginated list of entity watchlist screening reviews



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityReviewListResponse {
    /// List of entity watchlist screening reviews
    #[serde(rename = "entity_watchlist_screening_reviews")]
    pub entity_watchlist_screening_reviews: Vec<crate::models::EntityWatchlistScreeningReview>,
    /// An identifier that determines which page of results you receive.
    #[serde(rename = "next_cursor", deserialize_with = "Option::deserialize")]
    pub next_cursor: Option<String>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl WatchlistScreeningEntityReviewListResponse {
    /// Paginated list of entity watchlist screening reviews
    pub fn new(entity_watchlist_screening_reviews: Vec<crate::models::EntityWatchlistScreeningReview>, next_cursor: Option<String>, request_id: String) -> WatchlistScreeningEntityReviewListResponse {
        WatchlistScreeningEntityReviewListResponse {
            entity_watchlist_screening_reviews,
            next_cursor,
            request_id,
        }
    }
}


