/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MatchSummary : Summary object reflecting the match result of the associated data



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchSummary {
    #[serde(rename = "summary")]
    pub summary: crate::models::MatchSummaryCode,
}

impl MatchSummary {
    /// Summary object reflecting the match result of the associated data
    pub fn new(summary: crate::models::MatchSummaryCode) -> MatchSummary {
        MatchSummary {
            summary,
        }
    }
}

