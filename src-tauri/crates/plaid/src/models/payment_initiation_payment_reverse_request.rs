/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationPaymentReverseRequest : PaymentInitiationPaymentReverseRequest defines the request schema for `/payment_initiation/payment/reverse`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentReverseRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The ID of the payment to reverse
    #[serde(rename = "payment_id")]
    pub payment_id: String,
    /// A random key provided by the client, per unique wallet transaction. Maximum of 128 characters.  The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. If a request to execute a wallet transaction fails due to a network connection error, then after a minimum delay of one minute, you can retry the request with the same idempotency key to guarantee that only a single wallet transaction is created. If the request was successfully processed, it will prevent any transaction that uses the same idempotency key, and was received within 24 hours of the first request, from being processed.
    #[serde(rename = "idempotency_key")]
    pub idempotency_key: String,
    /// A reference for the refund. This must be an alphanumeric string with 6 to 18 characters and must not contain any special characters or spaces.
    #[serde(rename = "reference")]
    pub reference: String,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<Box<crate::models::PaymentAmountToRefund>>,
}

impl PaymentInitiationPaymentReverseRequest {
    /// PaymentInitiationPaymentReverseRequest defines the request schema for `/payment_initiation/payment/reverse`
    pub fn new(payment_id: String, idempotency_key: String, reference: String) -> PaymentInitiationPaymentReverseRequest {
        PaymentInitiationPaymentReverseRequest {
            client_id: None,
            secret: None,
            payment_id,
            idempotency_key,
            reference,
            amount: None,
        }
    }
}


