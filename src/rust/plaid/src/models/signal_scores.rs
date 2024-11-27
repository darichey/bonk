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

/// SignalScores : Risk scoring details broken down by risk category.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignalScores {
    #[serde(rename = "customer_initiated_return_risk", skip_serializing_if = "Option::is_none")]
    pub customer_initiated_return_risk: Option<Box<models::CustomerInitiatedReturnRisk>>,
    #[serde(rename = "bank_initiated_return_risk", skip_serializing_if = "Option::is_none")]
    pub bank_initiated_return_risk: Option<Box<models::BankInitiatedReturnRisk>>,
}

impl SignalScores {
    /// Risk scoring details broken down by risk category.
    pub fn new() -> SignalScores {
        SignalScores {
            customer_initiated_return_risk: None,
            bank_initiated_return_risk: None,
        }
    }
}

