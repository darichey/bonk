/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentConsentPeriodicAmount : Defines consent payments limitations per period.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentConsentPeriodicAmount {
    #[serde(rename = "amount")]
    pub amount: Box<crate::models::PaymentConsentPeriodicAmountAmount>,
    #[serde(rename = "interval")]
    pub interval: crate::models::PaymentConsentPeriodicInterval,
    #[serde(rename = "alignment")]
    pub alignment: crate::models::PaymentConsentPeriodicAlignment,
}

impl PaymentConsentPeriodicAmount {
    /// Defines consent payments limitations per period.
    pub fn new(amount: crate::models::PaymentConsentPeriodicAmountAmount, interval: crate::models::PaymentConsentPeriodicInterval, alignment: crate::models::PaymentConsentPeriodicAlignment) -> PaymentConsentPeriodicAmount {
        PaymentConsentPeriodicAmount {
            amount: Box::new(amount),
            interval,
            alignment,
        }
    }
}


