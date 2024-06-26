/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// KycCheckNameSummary : Result summary object specifying how the `name` field matched.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KycCheckNameSummary {
    #[serde(rename = "summary")]
    pub summary: crate::models::MatchSummaryCode,
}

impl KycCheckNameSummary {
    /// Result summary object specifying how the `name` field matched.
    pub fn new(summary: crate::models::MatchSummaryCode) -> KycCheckNameSummary {
        KycCheckNameSummary {
            summary,
        }
    }
}


