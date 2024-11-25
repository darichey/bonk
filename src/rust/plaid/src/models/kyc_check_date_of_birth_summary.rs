/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// KycCheckDateOfBirthSummary : Result summary object specifying how the `date_of_birth` field matched.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct KycCheckDateOfBirthSummary {
    #[serde(rename = "summary")]
    pub summary: crate::models::MatchSummaryCode,
    #[serde(rename = "day", skip_serializing_if = "Option::is_none")]
    pub day: Option<crate::models::HiddenMatchSummaryCode>,
    #[serde(rename = "month", skip_serializing_if = "Option::is_none")]
    pub month: Option<crate::models::HiddenMatchSummaryCode>,
    #[serde(rename = "year", skip_serializing_if = "Option::is_none")]
    pub year: Option<crate::models::HiddenMatchSummaryCode>,
}

impl KycCheckDateOfBirthSummary {
    /// Result summary object specifying how the `date_of_birth` field matched.
    pub fn new(summary: crate::models::MatchSummaryCode) -> KycCheckDateOfBirthSummary {
        KycCheckDateOfBirthSummary {
            summary,
            day: None,
            month: None,
            year: None,
        }
    }
}


