/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProcessorApexProcessorTokenCreateRequest : ProcessorApexProcessorTokenCreateRequest defines the request schema for `/processor/apex/processor_token/create`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProcessorApexProcessorTokenCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The access token associated with the Item data is being requested for.
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// The `account_id` value obtained from the `onSuccess` callback in Link
    #[serde(rename = "account_id")]
    pub account_id: String,
}

impl ProcessorApexProcessorTokenCreateRequest {
    /// ProcessorApexProcessorTokenCreateRequest defines the request schema for `/processor/apex/processor_token/create`
    pub fn new(access_token: String, account_id: String) -> ProcessorApexProcessorTokenCreateRequest {
        ProcessorApexProcessorTokenCreateRequest {
            client_id: None,
            secret: None,
            access_token,
            account_id,
        }
    }
}


