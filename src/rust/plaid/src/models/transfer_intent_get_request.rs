/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferIntentGetRequest : Defines the request schema for `/transfer/intent/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferIntentGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Plaid's unique identifier for a transfer intent object.
    #[serde(rename = "transfer_intent_id")]
    pub transfer_intent_id: String,
}

impl TransferIntentGetRequest {
    /// Defines the request schema for `/transfer/intent/get`
    pub fn new(transfer_intent_id: String) -> TransferIntentGetRequest {
        TransferIntentGetRequest {
            client_id: None,
            secret: None,
            transfer_intent_id,
        }
    }
}


