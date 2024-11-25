/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WalletGetRequest : WalletGetRequest defines the request schema for `/wallet/get`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WalletGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The ID of the e-wallet
    #[serde(rename = "wallet_id")]
    pub wallet_id: String,
}

impl WalletGetRequest {
    /// WalletGetRequest defines the request schema for `/wallet/get`
    pub fn new(wallet_id: String) -> WalletGetRequest {
        WalletGetRequest {
            client_id: None,
            secret: None,
            wallet_id,
        }
    }
}


