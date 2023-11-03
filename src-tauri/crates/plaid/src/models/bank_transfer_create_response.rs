/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BankTransferCreateResponse : Defines the response schema for `/bank_transfer/create`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BankTransferCreateResponse {
    #[serde(rename = "bank_transfer")]
    pub bank_transfer: crate::models::BankTransfer,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl BankTransferCreateResponse {
    /// Defines the response schema for `/bank_transfer/create`
    pub fn new(bank_transfer: crate::models::BankTransfer, request_id: String) -> BankTransferCreateResponse {
        BankTransferCreateResponse {
            bank_transfer,
            request_id,
        }
    }
}


