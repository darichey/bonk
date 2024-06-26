/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WatchlistScreeningEntityHistoryListRequest : Request input for listing changes to entity watchlist screenings



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityHistoryListRequest {
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// ID of the associated entity screening.
    #[serde(rename = "entity_watchlist_screening_id")]
    pub entity_watchlist_screening_id: String,
    /// An identifier that determines which page of results you receive.
    #[serde(rename = "cursor", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Option<String>>,
}

impl WatchlistScreeningEntityHistoryListRequest {
    /// Request input for listing changes to entity watchlist screenings
    pub fn new(entity_watchlist_screening_id: String) -> WatchlistScreeningEntityHistoryListRequest {
        WatchlistScreeningEntityHistoryListRequest {
            secret: None,
            client_id: None,
            entity_watchlist_screening_id,
            cursor: None,
        }
    }
}


