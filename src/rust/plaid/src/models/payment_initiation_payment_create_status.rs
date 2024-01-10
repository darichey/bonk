/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationPaymentCreateStatus : For a payment returned by this endpoint, there is only one possible value:  `PAYMENT_STATUS_INPUT_NEEDED`: The initial phase of the payment

/// For a payment returned by this endpoint, there is only one possible value:  `PAYMENT_STATUS_INPUT_NEEDED`: The initial phase of the payment
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentInitiationPaymentCreateStatus {
    #[serde(rename = "PAYMENT_STATUS_INPUT_NEEDED")]
    PaymentStatusInputNeeded,

}

impl ToString for PaymentInitiationPaymentCreateStatus {
    fn to_string(&self) -> String {
        match self {
            Self::PaymentStatusInputNeeded => String::from("PAYMENT_STATUS_INPUT_NEEDED"),
        }
    }
}

impl Default for PaymentInitiationPaymentCreateStatus {
    fn default() -> PaymentInitiationPaymentCreateStatus {
        Self::PaymentStatusInputNeeded
    }
}



