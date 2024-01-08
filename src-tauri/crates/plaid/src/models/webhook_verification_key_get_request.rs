/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WebhookVerificationKeyGetRequest : WebhookVerificationKeyGetRequest defines the request schema for `/webhook_verification_key/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookVerificationKeyGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The key ID ( `kid` ) from the JWT header.
    #[serde(rename = "key_id")]
    pub key_id: String,
}

impl WebhookVerificationKeyGetRequest {
    /// WebhookVerificationKeyGetRequest defines the request schema for `/webhook_verification_key/get`
    pub fn new(key_id: String) -> WebhookVerificationKeyGetRequest {
        WebhookVerificationKeyGetRequest {
            client_id: None,
            secret: None,
            key_id,
        }
    }
}


