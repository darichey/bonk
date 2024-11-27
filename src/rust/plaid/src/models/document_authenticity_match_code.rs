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

/// DocumentAuthenticityMatchCode : High level summary of whether the document in the provided image matches the formatting rules and security checks for the associated jurisdiction.  For example, most identity documents have formatting rules like the following:   The image of the person's face must have a certain contrast in order to highlight skin tone   The subject in the document's image must remove eye glasses and pose in a certain way   The informational fields (name, date of birth, ID number, etc.) must be colored and aligned according to specific rules   Security features like watermarks and background patterns must be present  So a `match` status for this field indicates that the document in the provided image seems to conform to the various formatting and security rules associated with the detected document.
/// High level summary of whether the document in the provided image matches the formatting rules and security checks for the associated jurisdiction.  For example, most identity documents have formatting rules like the following:   The image of the person's face must have a certain contrast in order to highlight skin tone   The subject in the document's image must remove eye glasses and pose in a certain way   The informational fields (name, date of birth, ID number, etc.) must be colored and aligned according to specific rules   Security features like watermarks and background patterns must be present  So a `match` status for this field indicates that the document in the provided image seems to conform to the various formatting and security rules associated with the detected document.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DocumentAuthenticityMatchCode {
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "partial_match")]
    PartialMatch,
    #[serde(rename = "no_match")]
    NoMatch,
    #[serde(rename = "no_data")]
    NoData,

}

impl std::fmt::Display for DocumentAuthenticityMatchCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Match => write!(f, "match"),
            Self::PartialMatch => write!(f, "partial_match"),
            Self::NoMatch => write!(f, "no_match"),
            Self::NoData => write!(f, "no_data"),
        }
    }
}

impl Default for DocumentAuthenticityMatchCode {
    fn default() -> DocumentAuthenticityMatchCode {
        Self::Match
    }
}

