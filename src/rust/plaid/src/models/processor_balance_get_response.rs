/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProcessorBalanceGetResponse : ProcessorBalanceGetResponse defines the response schema for `/processor/balance/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessorBalanceGetResponse {
    #[serde(rename = "account")]
    pub account: crate::models::AccountBase,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl ProcessorBalanceGetResponse {
    /// ProcessorBalanceGetResponse defines the response schema for `/processor/balance/get`
    pub fn new(account: crate::models::AccountBase, request_id: String) -> ProcessorBalanceGetResponse {
        ProcessorBalanceGetResponse {
            account,
            request_id,
        }
    }
}


