/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferBalanceGetResponse : Defines the response schema for `/transfer/balance/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferBalanceGetResponse {
    #[serde(rename = "balance")]
    pub balance: crate::models::TransferBalance,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransferBalanceGetResponse {
    /// Defines the response schema for `/transfer/balance/get`
    pub fn new(balance: crate::models::TransferBalance, request_id: String) -> TransferBalanceGetResponse {
        TransferBalanceGetResponse {
            balance,
            request_id,
        }
    }
}


