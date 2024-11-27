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

/// CreditSessionBankIncomeStatus : Status of the Bank Income Link session.  `APPROVED`: User has approved and verified their income  `NO_DEPOSITS_FOUND`: We attempted, but were unable to find any income in the connected account.  `USER_REPORTED_NO_INCOME`: The user explicitly indicated that they don't receive income in the connected account.  `STARTED`: The user began the bank income portion of the link flow.  `INTERNAL_ERROR`: The user encountered an internal error.
/// Status of the Bank Income Link session.  `APPROVED`: User has approved and verified their income  `NO_DEPOSITS_FOUND`: We attempted, but were unable to find any income in the connected account.  `USER_REPORTED_NO_INCOME`: The user explicitly indicated that they don't receive income in the connected account.  `STARTED`: The user began the bank income portion of the link flow.  `INTERNAL_ERROR`: The user encountered an internal error.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreditSessionBankIncomeStatus {
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "NO_DEPOSITS_FOUND")]
    NoDepositsFound,
    #[serde(rename = "USER_REPORTED_NO_INCOME")]
    UserReportedNoIncome,

}

impl std::fmt::Display for CreditSessionBankIncomeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Approved => write!(f, "APPROVED"),
            Self::NoDepositsFound => write!(f, "NO_DEPOSITS_FOUND"),
            Self::UserReportedNoIncome => write!(f, "USER_REPORTED_NO_INCOME"),
        }
    }
}

impl Default for CreditSessionBankIncomeStatus {
    fn default() -> CreditSessionBankIncomeStatus {
        Self::Approved
    }
}

