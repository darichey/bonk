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

/// PayStubDistributionBreakdown : Information about the accounts that the payment was distributed to.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayStubDistributionBreakdown {
    /// Name of the account for the given distribution.
    #[serde(rename = "account_name", deserialize_with = "Option::deserialize")]
    pub account_name: Option<String>,
    /// The name of the bank that the payment is being deposited to.
    #[serde(rename = "bank_name", deserialize_with = "Option::deserialize")]
    pub bank_name: Option<String>,
    /// The amount distributed to this account.
    #[serde(rename = "current_amount", deserialize_with = "Option::deserialize")]
    pub current_amount: Option<f64>,
    /// The ISO-4217 currency code of the net pay. Always `null` if `unofficial_currency_code` is non-null.
    #[serde(rename = "iso_currency_code", deserialize_with = "Option::deserialize")]
    pub iso_currency_code: Option<String>,
    /// The last 2-4 alphanumeric characters of an account's official account number.
    #[serde(rename = "mask", deserialize_with = "Option::deserialize")]
    pub mask: Option<String>,
    /// Type of the account that the paystub was sent to (e.g. 'checking').
    #[serde(rename = "type", deserialize_with = "Option::deserialize")]
    pub r#type: Option<String>,
    /// The unofficial currency code associated with the net pay. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.
    #[serde(rename = "unofficial_currency_code", deserialize_with = "Option::deserialize")]
    pub unofficial_currency_code: Option<String>,
}

impl PayStubDistributionBreakdown {
    /// Information about the accounts that the payment was distributed to.
    pub fn new(account_name: Option<String>, bank_name: Option<String>, current_amount: Option<f64>, iso_currency_code: Option<String>, mask: Option<String>, r#type: Option<String>, unofficial_currency_code: Option<String>) -> PayStubDistributionBreakdown {
        PayStubDistributionBreakdown {
            account_name,
            bank_name,
            current_amount,
            iso_currency_code,
            mask,
            r#type,
            unofficial_currency_code,
        }
    }
}

