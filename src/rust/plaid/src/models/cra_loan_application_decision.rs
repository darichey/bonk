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

/// CraLoanApplicationDecision : The decision of the loan application.
/// The decision of the loan application.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CraLoanApplicationDecision {
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "DECLINED")]
    Declined,
    #[serde(rename = "OTHER")]
    Other,

}

impl std::fmt::Display for CraLoanApplicationDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Approved => write!(f, "APPROVED"),
            Self::Declined => write!(f, "DECLINED"),
            Self::Other => write!(f, "OTHER"),
        }
    }
}

impl Default for CraLoanApplicationDecision {
    fn default() -> CraLoanApplicationDecision {
        Self::Approved
    }
}

