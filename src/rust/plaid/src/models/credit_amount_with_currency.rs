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

/// CreditAmountWithCurrency : This contains an amount, denominated in the currency specified by either `iso_currency_code` or `unofficial_currency_code`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditAmountWithCurrency {
    /// Value of amount with up to 2 decimal places.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// The ISO 4217 currency code of the amount or balance.
    #[serde(rename = "iso_currency_code", deserialize_with = "Option::deserialize")]
    pub iso_currency_code: Option<String>,
    /// The unofficial currency code associated with the amount or balance. Always `null` if `iso_currency_code` is non-null. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.
    #[serde(rename = "unofficial_currency_code", deserialize_with = "Option::deserialize")]
    pub unofficial_currency_code: Option<String>,
}

impl CreditAmountWithCurrency {
    /// This contains an amount, denominated in the currency specified by either `iso_currency_code` or `unofficial_currency_code`
    pub fn new(amount: f64, iso_currency_code: Option<String>, unofficial_currency_code: Option<String>) -> CreditAmountWithCurrency {
        CreditAmountWithCurrency {
            amount,
            iso_currency_code,
            unofficial_currency_code,
        }
    }
}

