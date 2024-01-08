/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FraudAmount : The amount and currency of the fraud or attempted fraud. `fraud_amount` should be omitted to indicate an unknown fraud amount.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FraudAmount {
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: crate::models::IsoCurrencyCode,
    /// The amount value. This value can be 0 to indicate no money was lost. Must not contain more than two digits of precision (e.g., `1.23`).
    #[serde(rename = "value")]
    pub value: f64,
}

impl FraudAmount {
    /// The amount and currency of the fraud or attempted fraud. `fraud_amount` should be omitted to indicate an unknown fraud amount.
    pub fn new(iso_currency_code: crate::models::IsoCurrencyCode, value: f64) -> FraudAmount {
        FraudAmount {
            iso_currency_code,
            value,
        }
    }
}


