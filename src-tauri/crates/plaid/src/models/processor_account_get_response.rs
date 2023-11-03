/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProcessorAccountGetResponse : ProcessorAccountGetResponse defines the response schema for `/processor/account/get`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProcessorAccountGetResponse {
    #[serde(rename = "account")]
    pub account: crate::models::AccountBase,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl ProcessorAccountGetResponse {
    /// ProcessorAccountGetResponse defines the response schema for `/processor/account/get`
    pub fn new(account: crate::models::AccountBase, request_id: String) -> ProcessorAccountGetResponse {
        ProcessorAccountGetResponse {
            account,
            request_id,
        }
    }
}


