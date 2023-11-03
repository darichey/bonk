/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BankTransferGetResponse : Defines the response schema for `/bank_transfer/get`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BankTransferGetResponse {
    #[serde(rename = "bank_transfer")]
    pub bank_transfer: crate::models::BankTransfer,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl BankTransferGetResponse {
    /// Defines the response schema for `/bank_transfer/get`
    pub fn new(bank_transfer: crate::models::BankTransfer, request_id: String) -> BankTransferGetResponse {
        BankTransferGetResponse {
            bank_transfer,
            request_id,
        }
    }
}


