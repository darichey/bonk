/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// KycCheckDateOfBirthSummary : Result summary object specifying how the `date_of_birth` field matched.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KycCheckDateOfBirthSummary {
    #[serde(rename = "summary")]
    pub summary: crate::models::MatchSummaryCode,
}

impl KycCheckDateOfBirthSummary {
    /// Result summary object specifying how the `date_of_birth` field matched.
    pub fn new(summary: crate::models::MatchSummaryCode) -> KycCheckDateOfBirthSummary {
        KycCheckDateOfBirthSummary {
            summary,
        }
    }
}


