/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationConsentCreateRequest : PaymentInitiationConsentCreateRequest defines the request schema for `/payment_initiation/consent/create`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentInitiationConsentCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The ID of the recipient the payment consent is for. The created consent can be used to transfer funds to this recipient only.
    #[serde(rename = "recipient_id")]
    pub recipient_id: String,
    /// A reference for the payment consent. This must be an alphanumeric string with at most 18 characters and must not contain any special characters.
    #[serde(rename = "reference")]
    pub reference: String,
    /// An array of payment consent scopes.
    #[serde(rename = "scopes")]
    pub scopes: Vec<crate::models::PaymentInitiationConsentScope>,
    #[serde(rename = "constraints")]
    pub constraints: Box<crate::models::PaymentInitiationConsentConstraints>,
    #[serde(rename = "options", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub options: Option<Option<Box<crate::models::ExternalPaymentInitiationConsentOptions>>>,
}

impl PaymentInitiationConsentCreateRequest {
    /// PaymentInitiationConsentCreateRequest defines the request schema for `/payment_initiation/consent/create`
    pub fn new(recipient_id: String, reference: String, scopes: Vec<crate::models::PaymentInitiationConsentScope>, constraints: crate::models::PaymentInitiationConsentConstraints) -> PaymentInitiationConsentCreateRequest {
        PaymentInitiationConsentCreateRequest {
            client_id: None,
            secret: None,
            recipient_id,
            reference,
            scopes,
            constraints: Box::new(constraints),
            options: None,
        }
    }
}


