/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenCreateRequestPaymentInitiation : Specifies options for initializing Link for use with the Payment Initiation (Europe) product. This field is required if `payment_initiation` is included in the `products` array. Either `payment_id` or `consent_id` must be provided.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestPaymentInitiation {
    /// The `payment_id` provided by the `/payment_initiation/payment/create` endpoint.
    #[serde(rename = "payment_id", skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    /// The `consent_id` provided by the `/payment_initiation/consent/create` endpoint.
    #[serde(rename = "consent_id", skip_serializing_if = "Option::is_none")]
    pub consent_id: Option<String>,
}

impl LinkTokenCreateRequestPaymentInitiation {
    /// Specifies options for initializing Link for use with the Payment Initiation (Europe) product. This field is required if `payment_initiation` is included in the `products` array. Either `payment_id` or `consent_id` must be provided.
    pub fn new() -> LinkTokenCreateRequestPaymentInitiation {
        LinkTokenCreateRequestPaymentInitiation {
            payment_id: None,
            consent_id: None,
        }
    }
}

