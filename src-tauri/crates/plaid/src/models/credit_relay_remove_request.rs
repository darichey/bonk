/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditRelayRemoveRequest : CreditRelayRemoveRequest defines the request schema for `/credit/relay/remove`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditRelayRemoveRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The `relay_token` you would like to revoke.
    #[serde(rename = "relay_token")]
    pub relay_token: String,
}

impl CreditRelayRemoveRequest {
    /// CreditRelayRemoveRequest defines the request schema for `/credit/relay/remove`
    pub fn new(relay_token: String) -> CreditRelayRemoveRequest {
        CreditRelayRemoveRequest {
            client_id: None,
            secret: None,
            relay_token,
        }
    }
}


