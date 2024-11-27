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

/// BalancePlusRiskLevel : A five-tier risk assessment for the transaction, based on the probability distribution of ACH returns, measured by the incident rate.   Each tier corresponds to a distribution with a different mean (average) probability.   `HIGH`: The mean probability of ACH return risk is above 40%. `MEDIUM_HIGH`: The mean probability of ACH return risk is 15%-40%.  `MEDIUM`: The mean probability of ACH return risk is 5-10%. `MEDIUM_LOW`: The mean probability of ACH return risk is 1%-2%. `LOW`: The mean probability of ACH return risk is below 1%.   Note that these tiers correspond to probability *distributions* and not to discrete probabilities.   These tier definitions are specific to Balance Plus and do not apply to risk tiers generated by other Plaid endpoints. 
/// A five-tier risk assessment for the transaction, based on the probability distribution of ACH returns, measured by the incident rate.   Each tier corresponds to a distribution with a different mean (average) probability.   `HIGH`: The mean probability of ACH return risk is above 40%. `MEDIUM_HIGH`: The mean probability of ACH return risk is 15%-40%.  `MEDIUM`: The mean probability of ACH return risk is 5-10%. `MEDIUM_LOW`: The mean probability of ACH return risk is 1%-2%. `LOW`: The mean probability of ACH return risk is below 1%.   Note that these tiers correspond to probability *distributions* and not to discrete probabilities.   These tier definitions are specific to Balance Plus and do not apply to risk tiers generated by other Plaid endpoints. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BalancePlusRiskLevel {
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "MEDIUM_HIGH")]
    MediumHigh,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "MEDIUM_LOW")]
    MediumLow,
    #[serde(rename = "LOW")]
    Low,

}

impl std::fmt::Display for BalancePlusRiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::High => write!(f, "HIGH"),
            Self::MediumHigh => write!(f, "MEDIUM_HIGH"),
            Self::Medium => write!(f, "MEDIUM"),
            Self::MediumLow => write!(f, "MEDIUM_LOW"),
            Self::Low => write!(f, "LOW"),
        }
    }
}

impl Default for BalancePlusRiskLevel {
    fn default() -> BalancePlusRiskLevel {
        Self::High
    }
}

