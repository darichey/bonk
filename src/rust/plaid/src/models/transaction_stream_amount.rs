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

/// TransactionStreamAmount : Object with data pertaining to an amount on the transaction stream.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionStreamAmount {
    /// Represents the numerical value of an amount.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// The ISO-4217 currency code of the amount. Always `null` if `unofficial_currency_code` is non-`null`.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.
    #[serde(rename = "iso_currency_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<Option<String>>,
    /// The unofficial currency code of the amount. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.
    #[serde(rename = "unofficial_currency_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<Option<String>>,
}

impl TransactionStreamAmount {
    /// Object with data pertaining to an amount on the transaction stream.
    pub fn new() -> TransactionStreamAmount {
        TransactionStreamAmount {
            amount: None,
            iso_currency_code: None,
            unofficial_currency_code: None,
        }
    }
}

