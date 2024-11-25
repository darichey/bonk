/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DocumentDateOfBirthMatchCode : A match summary describing the cross comparison between the subject's date of birth, extracted from the document image, and the date of birth they separately provided to the identity verification attempt.

/// A match summary describing the cross comparison between the subject's date of birth, extracted from the document image, and the date of birth they separately provided to the identity verification attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DocumentDateOfBirthMatchCode {
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "partial_match")]
    PartialMatch,
    #[serde(rename = "no_match")]
    NoMatch,
    #[serde(rename = "no_data")]
    NoData,

}

impl ToString for DocumentDateOfBirthMatchCode {
    fn to_string(&self) -> String {
        match self {
            Self::Match => String::from("match"),
            Self::PartialMatch => String::from("partial_match"),
            Self::NoMatch => String::from("no_match"),
            Self::NoData => String::from("no_data"),
        }
    }
}

impl Default for DocumentDateOfBirthMatchCode {
    fn default() -> DocumentDateOfBirthMatchCode {
        Self::Match
    }
}




