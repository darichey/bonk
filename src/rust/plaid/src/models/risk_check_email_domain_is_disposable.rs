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

/// RiskCheckEmailDomainIsDisposable : Indicates whether the email domain is listed as disposable if known. Disposable domains are often used to create email addresses that are part of a fake set of user details.
/// Indicates whether the email domain is listed as disposable if known. Disposable domains are often used to create email addresses that are part of a fake set of user details.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RiskCheckEmailDomainIsDisposable {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "no_data")]
    NoData,

}

impl std::fmt::Display for RiskCheckEmailDomainIsDisposable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Yes => write!(f, "yes"),
            Self::No => write!(f, "no"),
            Self::NoData => write!(f, "no_data"),
        }
    }
}

impl Default for RiskCheckEmailDomainIsDisposable {
    fn default() -> RiskCheckEmailDomainIsDisposable {
        Self::Yes
    }
}

