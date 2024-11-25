/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// StandaloneCurrencyCodeList : The following currency codes are supported by Plaid.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StandaloneCurrencyCodeList {
    /// Plaid supports all ISO 4217 currency codes.
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
    /// List of unofficial currency codes
    #[serde(rename = "unofficial_currency_code")]
    pub unofficial_currency_code: String,
}

impl StandaloneCurrencyCodeList {
    /// The following currency codes are supported by Plaid.
    pub fn new(iso_currency_code: String, unofficial_currency_code: String) -> StandaloneCurrencyCodeList {
        StandaloneCurrencyCodeList {
            iso_currency_code,
            unofficial_currency_code,
        }
    }
}


