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

/// EntityWatchlistScreeningHit : Data from a government watchlist that has been attached to the screening.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityWatchlistScreeningHit {
    /// ID of the associated entity screening hit.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "review_status")]
    pub review_status: models::WatchlistScreeningHitStatus,
    /// An ISO8601 formatted timestamp.
    #[serde(rename = "first_active")]
    pub first_active: String,
    /// An ISO8601 formatted timestamp.
    #[serde(rename = "inactive_since", deserialize_with = "Option::deserialize")]
    pub inactive_since: Option<String>,
    /// An ISO8601 formatted timestamp.
    #[serde(rename = "historical_since", deserialize_with = "Option::deserialize")]
    pub historical_since: Option<String>,
    #[serde(rename = "list_code")]
    pub list_code: models::EntityWatchlistCode,
    /// A universal identifier for a watchlist individual that is stable across searches and updates.
    #[serde(rename = "plaid_uid")]
    pub plaid_uid: String,
    /// The identifier provided by the source sanction or watchlist. When one is not provided by the source, this is `null`.
    #[serde(rename = "source_uid", deserialize_with = "Option::deserialize")]
    pub source_uid: Option<String>,
    #[serde(rename = "analysis", skip_serializing_if = "Option::is_none")]
    pub analysis: Option<models::EntityScreeningHitAnalysis>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<models::EntityScreeningHitData>,
}

impl EntityWatchlistScreeningHit {
    /// Data from a government watchlist that has been attached to the screening.
    pub fn new(id: String, review_status: models::WatchlistScreeningHitStatus, first_active: String, inactive_since: Option<String>, historical_since: Option<String>, list_code: models::EntityWatchlistCode, plaid_uid: String, source_uid: Option<String>) -> EntityWatchlistScreeningHit {
        EntityWatchlistScreeningHit {
            id,
            review_status,
            first_active,
            inactive_since,
            historical_since,
            list_code,
            plaid_uid,
            source_uid,
            analysis: None,
            data: None,
        }
    }
}

