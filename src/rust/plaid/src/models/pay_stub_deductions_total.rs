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

/// PayStubDeductionsTotal : An object representing the total deductions for the pay period
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayStubDeductionsTotal {
    /// Raw amount of the deduction
    #[serde(rename = "current_amount", deserialize_with = "Option::deserialize")]
    pub current_amount: Option<f64>,
    /// The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    #[serde(rename = "iso_currency_code", deserialize_with = "Option::deserialize")]
    pub iso_currency_code: Option<String>,
    /// The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.
    #[serde(rename = "unofficial_currency_code", deserialize_with = "Option::deserialize")]
    pub unofficial_currency_code: Option<String>,
    /// The year-to-date total amount of the deductions
    #[serde(rename = "ytd_amount", deserialize_with = "Option::deserialize")]
    pub ytd_amount: Option<f64>,
}

impl PayStubDeductionsTotal {
    /// An object representing the total deductions for the pay period
    pub fn new(current_amount: Option<f64>, iso_currency_code: Option<String>, unofficial_currency_code: Option<String>, ytd_amount: Option<f64>) -> PayStubDeductionsTotal {
        PayStubDeductionsTotal {
            current_amount,
            iso_currency_code,
            unofficial_currency_code,
            ytd_amount,
        }
    }
}

