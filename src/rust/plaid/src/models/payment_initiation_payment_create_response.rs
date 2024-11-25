/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationPaymentCreateResponse : PaymentInitiationPaymentCreateResponse defines the response schema for `/payment_initiation/payment/create`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentCreateResponse {
    /// A unique ID identifying the payment
    #[serde(rename = "payment_id")]
    pub payment_id: String,
    #[serde(rename = "status")]
    pub status: crate::models::PaymentInitiationPaymentCreateStatus,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl PaymentInitiationPaymentCreateResponse {
    /// PaymentInitiationPaymentCreateResponse defines the response schema for `/payment_initiation/payment/create`
    pub fn new(payment_id: String, status: crate::models::PaymentInitiationPaymentCreateStatus, request_id: String) -> PaymentInitiationPaymentCreateResponse {
        PaymentInitiationPaymentCreateResponse {
            payment_id,
            status,
            request_id,
        }
    }
}


