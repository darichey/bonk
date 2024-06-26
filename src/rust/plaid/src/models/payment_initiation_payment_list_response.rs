/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationPaymentListResponse : PaymentInitiationPaymentListResponse defines the response schema for `/payment_initiation/payment/list`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentListResponse {
    /// An array of payments that have been created, associated with the given `client_id`.
    #[serde(rename = "payments")]
    pub payments: Vec<crate::models::PaymentInitiationPayment>,
    /// The value that, when used as the optional `cursor` parameter to `/payment_initiation/payment/list`, will return the next unreturned payment as its first payment.
    #[serde(rename = "next_cursor", deserialize_with = "Option::deserialize")]
    pub next_cursor: Option<String>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl PaymentInitiationPaymentListResponse {
    /// PaymentInitiationPaymentListResponse defines the response schema for `/payment_initiation/payment/list`
    pub fn new(payments: Vec<crate::models::PaymentInitiationPayment>, next_cursor: Option<String>, request_id: String) -> PaymentInitiationPaymentListResponse {
        PaymentInitiationPaymentListResponse {
            payments,
            next_cursor,
            request_id,
        }
    }
}


