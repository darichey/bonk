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

/// ScreeningHitNamesItems : Analyzed name information for the associated hit
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScreeningHitNamesItems {
    #[serde(rename = "analysis", skip_serializing_if = "Option::is_none")]
    pub analysis: Option<models::MatchSummary>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<models::IndividualScreeningHitNames>,
}

impl ScreeningHitNamesItems {
    /// Analyzed name information for the associated hit
    pub fn new() -> ScreeningHitNamesItems {
        ScreeningHitNamesItems {
            analysis: None,
            data: None,
        }
    }
}

