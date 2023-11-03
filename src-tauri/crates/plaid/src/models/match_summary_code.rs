/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MatchSummaryCode : An enum indicating the match type between data provided by user and data checked against an external data source.   `match` indicates that the provided input data was a strong match against external data.  `partial_match` indicates the data approximately matched against external data. For example, \"Knope\" vs. \"Knope-Wyatt\" for last name.  `no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.  `no_data` indicates that Plaid was unable to find external data to compare against the provided input data.  `no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.

/// An enum indicating the match type between data provided by user and data checked against an external data source.   `match` indicates that the provided input data was a strong match against external data.  `partial_match` indicates the data approximately matched against external data. For example, \"Knope\" vs. \"Knope-Wyatt\" for last name.  `no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.  `no_data` indicates that Plaid was unable to find external data to compare against the provided input data.  `no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MatchSummaryCode {
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "partial_match")]
    PartialMatch,
    #[serde(rename = "no_match")]
    NoMatch,
    #[serde(rename = "no_data")]
    NoData,
    #[serde(rename = "no_input")]
    NoInput,

}

impl ToString for MatchSummaryCode {
    fn to_string(&self) -> String {
        match self {
            Self::Match => String::from("match"),
            Self::PartialMatch => String::from("partial_match"),
            Self::NoMatch => String::from("no_match"),
            Self::NoData => String::from("no_data"),
            Self::NoInput => String::from("no_input"),
        }
    }
}

impl Default for MatchSummaryCode {
    fn default() -> MatchSummaryCode {
        Self::Match
    }
}




