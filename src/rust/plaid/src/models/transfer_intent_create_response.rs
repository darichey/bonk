/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferIntentCreateResponse : Defines the response schema for `/transfer/intent/create`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferIntentCreateResponse {
    #[serde(rename = "transfer_intent")]
    pub transfer_intent: crate::models::TransferIntentCreate,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransferIntentCreateResponse {
    /// Defines the response schema for `/transfer/intent/create`
    pub fn new(transfer_intent: crate::models::TransferIntentCreate, request_id: String) -> TransferIntentCreateResponse {
        TransferIntentCreateResponse {
            transfer_intent,
            request_id,
        }
    }
}


