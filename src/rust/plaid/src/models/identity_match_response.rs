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

/// IdentityMatchResponse : IdentityMatchResponse defines the response schema for `/identity/match`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityMatchResponse {
    /// The accounts for which Identity match has been requested
    #[serde(rename = "accounts")]
    pub accounts: Vec<models::AccountIdentityMatchScore>,
    #[serde(rename = "item")]
    pub item: models::Item,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl IdentityMatchResponse {
    /// IdentityMatchResponse defines the response schema for `/identity/match`
    pub fn new(accounts: Vec<models::AccountIdentityMatchScore>, item: models::Item, request_id: String) -> IdentityMatchResponse {
        IdentityMatchResponse {
            accounts,
            item,
            request_id,
        }
    }
}

