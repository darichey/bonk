/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentAmountToRefund : The amount and currency of a payment



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentAmountToRefund {
    #[serde(rename = "currency")]
    pub currency: crate::models::PaymentAmountCurrency,
    /// The amount of the payment. Must contain at most two digits of precision e.g. `1.23`.
    #[serde(rename = "value")]
    pub value: f64,
}

impl PaymentAmountToRefund {
    /// The amount and currency of a payment
    pub fn new(currency: crate::models::PaymentAmountCurrency, value: f64) -> PaymentAmountToRefund {
        PaymentAmountToRefund {
            currency,
            value,
        }
    }
}


