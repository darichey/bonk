/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferLedgerDepositResponse : Defines the response schema for `/transfer/ledger/deposit`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferLedgerDepositResponse {
    #[serde(rename = "sweep")]
    pub sweep: crate::models::TransferSweep,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransferLedgerDepositResponse {
    /// Defines the response schema for `/transfer/ledger/deposit`
    pub fn new(sweep: crate::models::TransferSweep, request_id: String) -> TransferLedgerDepositResponse {
        TransferLedgerDepositResponse {
            sweep,
            request_id,
        }
    }
}


