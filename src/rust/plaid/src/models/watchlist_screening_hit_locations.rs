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

/// WatchlistScreeningHitLocations : Location information for the associated individual watchlist hit
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchlistScreeningHitLocations {
    /// The full location string, potentially including elements like street, city, postal codes and country codes. Note that this is not necessarily a complete or well-formatted address.
    #[serde(rename = "full")]
    pub full: String,
    /// Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    #[serde(rename = "country")]
    pub country: String,
}

impl WatchlistScreeningHitLocations {
    /// Location information for the associated individual watchlist hit
    pub fn new(full: String, country: String) -> WatchlistScreeningHitLocations {
        WatchlistScreeningHitLocations {
            full,
            country,
        }
    }
}

