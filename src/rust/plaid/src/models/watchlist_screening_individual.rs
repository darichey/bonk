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

/// WatchlistScreeningIndividual : The screening object allows you to represent a customer in your system, update their profile, and search for them on various watchlists. Note: Rejected customers will not receive new hits, regardless of program configuration.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividual {
    /// ID of the associated screening.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "search_terms")]
    pub search_terms: models::WatchlistScreeningSearchTerms,
    /// ID of the associated user. To retrieve the email address or other details of the person corresponding to this id, use `/dashboard_user/get`.
    #[serde(rename = "assignee", deserialize_with = "Option::deserialize")]
    pub assignee: Option<String>,
    #[serde(rename = "status")]
    pub status: models::WatchlistScreeningStatus,
    /// A unique ID that identifies the end user in your system. This ID can also be used to associate user-specific data from other Plaid products. Financial Account Matching requires this field and the `/link/token/create` `client_user_id` to be consistent. Personally identifiable information, such as an email address or phone number, should not be used in the `client_user_id`.
    #[serde(rename = "client_user_id", deserialize_with = "Option::deserialize")]
    pub client_user_id: Option<String>,
    #[serde(rename = "audit_trail")]
    pub audit_trail: models::WatchlistScreeningAuditTrail,
}

impl WatchlistScreeningIndividual {
    /// The screening object allows you to represent a customer in your system, update their profile, and search for them on various watchlists. Note: Rejected customers will not receive new hits, regardless of program configuration.
    pub fn new(id: String, search_terms: models::WatchlistScreeningSearchTerms, assignee: Option<String>, status: models::WatchlistScreeningStatus, client_user_id: Option<String>, audit_trail: models::WatchlistScreeningAuditTrail) -> WatchlistScreeningIndividual {
        WatchlistScreeningIndividual {
            id,
            search_terms,
            assignee,
            status,
            client_user_id,
            audit_trail,
        }
    }
}

