/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProcessorIdentityGetResponse : ProcessorIdentityGetResponse defines the response schema for `/processor/identity/get`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProcessorIdentityGetResponse {
    #[serde(rename = "account")]
    pub account: Box<crate::models::AccountIdentity>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl ProcessorIdentityGetResponse {
    /// ProcessorIdentityGetResponse defines the response schema for `/processor/identity/get`
    pub fn new(account: crate::models::AccountIdentity, request_id: String) -> ProcessorIdentityGetResponse {
        ProcessorIdentityGetResponse {
            account: Box::new(account),
            request_id,
        }
    }
}


