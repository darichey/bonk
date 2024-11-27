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

/// WatchlistScreeningIndividualUpdateRequest : Request input for editing an individual watchlist screening
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualUpdateRequest {
    /// ID of the associated screening.
    #[serde(rename = "watchlist_screening_id")]
    pub watchlist_screening_id: String,
    #[serde(rename = "search_terms", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub search_terms: Option<Option<Box<models::UpdateIndividualScreeningRequestSearchTerms>>>,
    /// ID of the associated user. To retrieve the email address or other details of the person corresponding to this id, use `/dashboard_user/get`.
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::WatchlistScreeningStatus>,
    /// A unique ID that identifies the end user in your system. This ID can also be used to associate user-specific data from other Plaid products. Financial Account Matching requires this field and the `/link/token/create` `client_user_id` to be consistent. Personally identifiable information, such as an email address or phone number, should not be used in the `client_user_id`.
    #[serde(rename = "client_user_id", skip_serializing_if = "Option::is_none")]
    pub client_user_id: Option<String>,
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// A list of fields to reset back to null
    #[serde(rename = "reset_fields", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub reset_fields: Option<Option<Vec<models::WatchlistScreeningIndividualUpdateRequestResettableField>>>,
}

impl WatchlistScreeningIndividualUpdateRequest {
    /// Request input for editing an individual watchlist screening
    pub fn new(watchlist_screening_id: String) -> WatchlistScreeningIndividualUpdateRequest {
        WatchlistScreeningIndividualUpdateRequest {
            watchlist_screening_id,
            search_terms: None,
            assignee: None,
            status: None,
            client_user_id: None,
            client_id: None,
            secret: None,
            reset_fields: None,
        }
    }
}

