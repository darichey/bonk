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

/// OptionContract : Details about the option security.  For the Sandbox environment, this data is currently only available if the item is using a custom configuration object, and the `ticker` field of the custom security follows the [OCC Option Symbol](https://en.wikipedia.org/wiki/Option_symbol#The_OCC_Option_Symbol) standard with no spaces.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OptionContract {
    /// The type of this option contract. It is one of:  `put`: for Put option contracts  `call`: for Call option contracts
    #[serde(rename = "contract_type")]
    pub contract_type: String,
    /// The expiration date for this option contract, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    #[serde(rename = "expiration_date")]
    pub expiration_date: String,
    /// The strike price for this option contract, per share of security.
    #[serde(rename = "strike_price")]
    pub strike_price: f64,
    /// The ticker of the underlying security for this option contract.
    #[serde(rename = "underlying_security_ticker")]
    pub underlying_security_ticker: String,
}

impl OptionContract {
    /// Details about the option security.  For the Sandbox environment, this data is currently only available if the item is using a custom configuration object, and the `ticker` field of the custom security follows the [OCC Option Symbol](https://en.wikipedia.org/wiki/Option_symbol#The_OCC_Option_Symbol) standard with no spaces.
    pub fn new(contract_type: String, expiration_date: String, strike_price: f64, underlying_security_ticker: String) -> OptionContract {
        OptionContract {
            contract_type,
            expiration_date,
            strike_price,
            underlying_security_ticker,
        }
    }
}

