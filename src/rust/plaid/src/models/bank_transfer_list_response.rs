/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BankTransferListResponse : Defines the response schema for `/bank_transfer/list`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BankTransferListResponse {
    #[serde(rename = "bank_transfers")]
    pub bank_transfers: Vec<crate::models::BankTransfer>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl BankTransferListResponse {
    /// Defines the response schema for `/bank_transfer/list`
    pub fn new(bank_transfers: Vec<crate::models::BankTransfer>, request_id: String) -> BankTransferListResponse {
        BankTransferListResponse {
            bank_transfers,
            request_id,
        }
    }
}


