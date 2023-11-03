/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PayStubEarningsBreakdown : An object representing the earnings line items for the pay period.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PayStubEarningsBreakdown {
    /// Commonly used term to describe the earning line item.
    #[serde(rename = "canonical_description", deserialize_with = "Option::deserialize")]
    pub canonical_description: Option<String>,
    /// Raw amount of the earning line item.
    #[serde(rename = "current_amount", deserialize_with = "Option::deserialize")]
    pub current_amount: Option<f64>,
    /// Description of the earning line item.
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    /// Number of hours applicable for this earning.
    #[serde(rename = "hours", deserialize_with = "Option::deserialize")]
    pub hours: Option<f32>,
    /// The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    #[serde(rename = "iso_currency_code", deserialize_with = "Option::deserialize")]
    pub iso_currency_code: Option<String>,
    /// Hourly rate applicable for this earning.
    #[serde(rename = "rate", deserialize_with = "Option::deserialize")]
    pub rate: Option<f64>,
    /// The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.
    #[serde(rename = "unofficial_currency_code", deserialize_with = "Option::deserialize")]
    pub unofficial_currency_code: Option<String>,
    /// The year-to-date amount of the deduction.
    #[serde(rename = "ytd_amount", deserialize_with = "Option::deserialize")]
    pub ytd_amount: Option<f64>,
}

impl PayStubEarningsBreakdown {
    /// An object representing the earnings line items for the pay period.
    pub fn new(canonical_description: Option<String>, current_amount: Option<f64>, description: Option<String>, hours: Option<f32>, iso_currency_code: Option<String>, rate: Option<f64>, unofficial_currency_code: Option<String>, ytd_amount: Option<f64>) -> PayStubEarningsBreakdown {
        PayStubEarningsBreakdown {
            canonical_description,
            current_amount,
            description,
            hours,
            iso_currency_code,
            rate,
            unofficial_currency_code,
            ytd_amount,
        }
    }
}


