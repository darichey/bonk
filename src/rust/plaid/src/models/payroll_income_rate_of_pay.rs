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

/// PayrollIncomeRateOfPay : An object representing the rate at which an individual is paid.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayrollIncomeRateOfPay {
    /// The rate at which an employee is paid.
    #[serde(rename = "pay_rate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pay_rate: Option<Option<String>>,
    /// The amount at which an employee is paid.
    #[serde(rename = "pay_amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pay_amount: Option<Option<f64>>,
}

impl PayrollIncomeRateOfPay {
    /// An object representing the rate at which an individual is paid.
    pub fn new() -> PayrollIncomeRateOfPay {
        PayrollIncomeRateOfPay {
            pay_rate: None,
            pay_amount: None,
        }
    }
}

