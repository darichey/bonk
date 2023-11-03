/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferConfigurationGetRequest : Defines the request schema for `/transfer/configuration/get`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferConfigurationGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The Plaid client ID of the transfer originator. Should only be present if `client_id` is a [Platform customer](https://plaid.com/docs/transfer/application/#originators-vs-platforms).
    #[serde(rename = "originator_client_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub originator_client_id: Option<Option<String>>,
}

impl TransferConfigurationGetRequest {
    /// Defines the request schema for `/transfer/configuration/get`
    pub fn new() -> TransferConfigurationGetRequest {
        TransferConfigurationGetRequest {
            client_id: None,
            secret: None,
            originator_client_id: None,
        }
    }
}


