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

/// ConsentEventType : A broad categorization of the consent event.
/// A broad categorization of the consent event.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConsentEventType {
    #[serde(rename = "CONSENT_GRANTED")]
    Granted,
    #[serde(rename = "CONSENT_REVOKED")]
    Revoked,
    #[serde(rename = "CONSENT_UPDATED")]
    Updated,

}

impl std::fmt::Display for ConsentEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Granted => write!(f, "CONSENT_GRANTED"),
            Self::Revoked => write!(f, "CONSENT_REVOKED"),
            Self::Updated => write!(f, "CONSENT_UPDATED"),
        }
    }
}

impl Default for ConsentEventType {
    fn default() -> ConsentEventType {
        Self::Granted
    }
}

