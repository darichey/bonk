/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferIntentGetResponse : Defines the response schema for `/transfer/intent/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferIntentGetResponse {
    #[serde(rename = "transfer_intent")]
    pub transfer_intent: crate::models::TransferIntentGet,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransferIntentGetResponse {
    /// Defines the response schema for `/transfer/intent/get`
    pub fn new(transfer_intent: crate::models::TransferIntentGet, request_id: String) -> TransferIntentGetResponse {
        TransferIntentGetResponse {
            transfer_intent,
            request_id,
        }
    }
}


