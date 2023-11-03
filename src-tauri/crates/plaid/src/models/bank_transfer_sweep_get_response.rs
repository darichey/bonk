/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BankTransferSweepGetResponse : BankTransferSweepGetResponse defines the response schema for `/bank_transfer/sweep/get`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BankTransferSweepGetResponse {
    #[serde(rename = "sweep")]
    pub sweep: crate::models::BankTransferSweep,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl BankTransferSweepGetResponse {
    /// BankTransferSweepGetResponse defines the response schema for `/bank_transfer/sweep/get`
    pub fn new(sweep: crate::models::BankTransferSweep, request_id: String) -> BankTransferSweepGetResponse {
        BankTransferSweepGetResponse {
            sweep,
            request_id,
        }
    }
}


