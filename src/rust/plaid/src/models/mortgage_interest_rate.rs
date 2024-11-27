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

/// MortgageInterestRate : Object containing metadata about the interest rate for the mortgage.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MortgageInterestRate {
    /// Percentage value (interest rate of current mortgage, not APR) of interest payable on a loan.
    #[serde(rename = "percentage", deserialize_with = "Option::deserialize")]
    pub percentage: Option<f64>,
    /// The type of interest charged (fixed or variable).
    #[serde(rename = "type", deserialize_with = "Option::deserialize")]
    pub r#type: Option<String>,
}

impl MortgageInterestRate {
    /// Object containing metadata about the interest rate for the mortgage.
    pub fn new(percentage: Option<f64>, r#type: Option<String>) -> MortgageInterestRate {
        MortgageInterestRate {
            percentage,
            r#type,
        }
    }
}

