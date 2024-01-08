/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationConsentConstraints : Limitations that will be applied to payments initiated using the payment consent.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentInitiationConsentConstraints {
    #[serde(rename = "valid_date_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub valid_date_time: Option<Option<Box<crate::models::PaymentConsentValidDateTime>>>,
    #[serde(rename = "max_payment_amount")]
    pub max_payment_amount: Box<crate::models::PaymentConsentMaxPaymentAmount>,
    /// A list of amount limitations per period of time.
    #[serde(rename = "periodic_amounts")]
    pub periodic_amounts: Vec<crate::models::PaymentConsentPeriodicAmount>,
}

impl PaymentInitiationConsentConstraints {
    /// Limitations that will be applied to payments initiated using the payment consent.
    pub fn new(max_payment_amount: crate::models::PaymentConsentMaxPaymentAmount, periodic_amounts: Vec<crate::models::PaymentConsentPeriodicAmount>) -> PaymentInitiationConsentConstraints {
        PaymentInitiationConsentConstraints {
            valid_date_time: None,
            max_payment_amount: Box::new(max_payment_amount),
            periodic_amounts,
        }
    }
}


