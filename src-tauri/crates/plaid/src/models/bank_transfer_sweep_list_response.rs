/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BankTransferSweepListResponse : BankTransferSweepListResponse defines the response schema for `/bank_transfer/sweep/list`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BankTransferSweepListResponse {
    #[serde(rename = "sweeps")]
    pub sweeps: Vec<crate::models::BankTransferSweep>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl BankTransferSweepListResponse {
    /// BankTransferSweepListResponse defines the response schema for `/bank_transfer/sweep/list`
    pub fn new(sweeps: Vec<crate::models::BankTransferSweep>, request_id: String) -> BankTransferSweepListResponse {
        BankTransferSweepListResponse {
            sweeps,
            request_id,
        }
    }
}


