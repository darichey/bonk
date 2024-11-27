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

/// AssetReportInvestmentHolding : A securities holding at an institution.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetReportInvestmentHolding {
    /// The Plaid `account_id` associated with the holding.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The Plaid `security_id` associated with the holding. Security data is not specific to a user's account; any user who held the same security at the same financial institution at the same time would have identical security data. The `security_id` for the same security will typically be the same across different institutions, but this is not guaranteed. The `security_id` does not typically change, but may change if inherent details of the security change due to a corporate action, for example, in the event of a ticker symbol change or CUSIP change.
    #[serde(rename = "security_id")]
    pub security_id: String,
    /// The holding's trading symbol for publicly traded holdings, and otherwise a short identifier if available.
    #[serde(rename = "ticker_symbol", deserialize_with = "Option::deserialize")]
    pub ticker_symbol: Option<String>,
    /// The last price given by the institution for this security.
    #[serde(rename = "institution_price")]
    pub institution_price: f64,
    /// The date at which `institution_price` was current.
    #[serde(rename = "institution_price_as_of", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub institution_price_as_of: Option<Option<String>>,
    /// The value of the holding, as reported by the institution.
    #[serde(rename = "institution_value")]
    pub institution_value: f64,
    /// The original total value of the holding. This field is calculated by Plaid as the sum of the purchase price of all of the shares in the holding.
    #[serde(rename = "cost_basis", deserialize_with = "Option::deserialize")]
    pub cost_basis: Option<f64>,
    /// The total quantity of the asset held, as reported by the financial institution. If the security is an option, `quantity` will reflect the total number of options (typically the number of contracts multiplied by 100), not the number of contracts.
    #[serde(rename = "quantity")]
    pub quantity: f64,
    /// The ISO-4217 currency code of the holding. Always `null` if `unofficial_currency_code` is non-`null`.
    #[serde(rename = "iso_currency_code", deserialize_with = "Option::deserialize")]
    pub iso_currency_code: Option<String>,
    /// The unofficial currency code associated with the holding. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s. 
    #[serde(rename = "unofficial_currency_code", deserialize_with = "Option::deserialize")]
    pub unofficial_currency_code: Option<String>,
}

impl AssetReportInvestmentHolding {
    /// A securities holding at an institution.
    pub fn new(account_id: String, security_id: String, ticker_symbol: Option<String>, institution_price: f64, institution_value: f64, cost_basis: Option<f64>, quantity: f64, iso_currency_code: Option<String>, unofficial_currency_code: Option<String>) -> AssetReportInvestmentHolding {
        AssetReportInvestmentHolding {
            account_id,
            security_id,
            ticker_symbol,
            institution_price,
            institution_price_as_of: None,
            institution_value,
            cost_basis,
            quantity,
            iso_currency_code,
            unofficial_currency_code,
        }
    }
}

