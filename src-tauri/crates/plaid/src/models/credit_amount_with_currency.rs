/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditAmountWithCurrency : This contains an amount, denominated in the currency specified by either `iso_currency_code` or `unofficial_currency_code`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditAmountWithCurrency {
    /// Value of amount with up to 2 decimal places.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// The ISO 4217 currency code of the amount or balance.
    #[serde(rename = "iso_currency_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<Option<String>>,
    /// The unofficial currency code associated with the amount or balance. Always `null` if `iso_currency_code` is non-null. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.
    #[serde(rename = "unofficial_currency_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<Option<String>>,
}

impl CreditAmountWithCurrency {
    /// This contains an amount, denominated in the currency specified by either `iso_currency_code` or `unofficial_currency_code`
    pub fn new() -> CreditAmountWithCurrency {
        CreditAmountWithCurrency {
            amount: None,
            iso_currency_code: None,
            unofficial_currency_code: None,
        }
    }
}


