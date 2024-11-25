/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentProfileGetRequest : PaymentProfileGetRequest defines the request schema for `/payment_profile/get`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentProfileGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// A payment profile token associated with the Payment Profile data that is being requested.
    #[serde(rename = "payment_profile_token")]
    pub payment_profile_token: String,
}

impl PaymentProfileGetRequest {
    /// PaymentProfileGetRequest defines the request schema for `/payment_profile/get`
    pub fn new(payment_profile_token: String) -> PaymentProfileGetRequest {
        PaymentProfileGetRequest {
            client_id: None,
            secret: None,
            payment_profile_token,
        }
    }
}


