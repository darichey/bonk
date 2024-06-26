/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationMetadata : Metadata that captures what specific payment configurations an institution supports when making Payment Initiation requests.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentInitiationMetadata {
    /// Indicates whether the institution supports payments from a different country.
    #[serde(rename = "supports_international_payments")]
    pub supports_international_payments: bool,
    /// Indicates whether the institution supports SEPA Instant payments.
    #[serde(rename = "supports_sepa_instant")]
    pub supports_sepa_instant: bool,
    /// A mapping of currency to maximum payment amount (denominated in the smallest unit of currency) supported by the institution.  Example: `{\"GBP\": \"10000\"}` 
    #[serde(rename = "maximum_payment_amount")]
    pub maximum_payment_amount: ::std::collections::HashMap<String, String>,
    /// Indicates whether the institution supports returning refund details when initiating a payment.
    #[serde(rename = "supports_refund_details")]
    pub supports_refund_details: bool,
    #[serde(rename = "standing_order_metadata", deserialize_with = "Option::deserialize")]
    pub standing_order_metadata: Option<crate::models::PaymentInitiationStandingOrderMetadata>,
}

impl PaymentInitiationMetadata {
    /// Metadata that captures what specific payment configurations an institution supports when making Payment Initiation requests.
    pub fn new(supports_international_payments: bool, supports_sepa_instant: bool, maximum_payment_amount: ::std::collections::HashMap<String, String>, supports_refund_details: bool, standing_order_metadata: Option<crate::models::PaymentInitiationStandingOrderMetadata>) -> PaymentInitiationMetadata {
        PaymentInitiationMetadata {
            supports_international_payments,
            supports_sepa_instant,
            maximum_payment_amount,
            supports_refund_details,
            standing_order_metadata,
        }
    }
}


