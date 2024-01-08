/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentAmount : The amount and currency of a payment



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentAmount {
    #[serde(rename = "currency")]
    pub currency: crate::models::PaymentAmountCurrency,
    /// The amount of the payment. Must contain at most two digits of precision e.g. `1.23`. Minimum accepted value is `1`.
    #[serde(rename = "value")]
    pub value: f64,
}

impl PaymentAmount {
    /// The amount and currency of a payment
    pub fn new(currency: crate::models::PaymentAmountCurrency, value: f64) -> PaymentAmount {
        PaymentAmount {
            currency,
            value,
        }
    }
}


