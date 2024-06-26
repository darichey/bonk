/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationRecipientGetRequest : PaymentInitiationRecipientGetRequest defines the request schema for `/payment_initiation/recipient/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The ID of the recipient
    #[serde(rename = "recipient_id")]
    pub recipient_id: String,
}

impl PaymentInitiationRecipientGetRequest {
    /// PaymentInitiationRecipientGetRequest defines the request schema for `/payment_initiation/recipient/get`
    pub fn new(recipient_id: String) -> PaymentInitiationRecipientGetRequest {
        PaymentInitiationRecipientGetRequest {
            client_id: None,
            secret: None,
            recipient_id,
        }
    }
}


