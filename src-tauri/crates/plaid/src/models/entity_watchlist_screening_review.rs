/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EntityWatchlistScreeningReview : A review submitted by a team member for an entity watchlist screening. A review can be either a comment on the current screening state, actions taken against hits attached to the watchlist screening, or both.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EntityWatchlistScreeningReview {
    /// ID of the associated entity review.
    #[serde(rename = "id")]
    pub id: String,
    /// Hits marked as a true positive after thorough manual review. These hits will never recur or be updated once dismissed. In most cases, confirmed hits indicate that the customer should be rejected.
    #[serde(rename = "confirmed_hits")]
    pub confirmed_hits: Vec<String>,
    /// Hits marked as a false positive after thorough manual review. These hits will never recur or be updated once dismissed.
    #[serde(rename = "dismissed_hits")]
    pub dismissed_hits: Vec<String>,
    /// A comment submitted by a team member as part of reviewing a watchlist screening.
    #[serde(rename = "comment", deserialize_with = "Option::deserialize")]
    pub comment: Option<String>,
    #[serde(rename = "audit_trail")]
    pub audit_trail: crate::models::WatchlistScreeningAuditTrail,
}

impl EntityWatchlistScreeningReview {
    /// A review submitted by a team member for an entity watchlist screening. A review can be either a comment on the current screening state, actions taken against hits attached to the watchlist screening, or both.
    pub fn new(id: String, confirmed_hits: Vec<String>, dismissed_hits: Vec<String>, comment: Option<String>, audit_trail: crate::models::WatchlistScreeningAuditTrail) -> EntityWatchlistScreeningReview {
        EntityWatchlistScreeningReview {
            id,
            confirmed_hits,
            dismissed_hits,
            comment,
            audit_trail,
        }
    }
}


