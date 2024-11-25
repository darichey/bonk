/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferLedgerDistributeResponse : Defines the response schema for `/transfer/ledger/distribute`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferLedgerDistributeResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransferLedgerDistributeResponse {
    /// Defines the response schema for `/transfer/ledger/distribute`
    pub fn new(request_id: String) -> TransferLedgerDistributeResponse {
        TransferLedgerDistributeResponse {
            request_id,
        }
    }
}


