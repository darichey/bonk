/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferOriginatorCreateRequest : Defines the request schema for `/transfer/originator/create`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferOriginatorCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The company name of the end customer being created. This will be displayed in public-facing surfaces, e.g. Plaid Dashboard.
    #[serde(rename = "company_name")]
    pub company_name: String,
}

impl TransferOriginatorCreateRequest {
    /// Defines the request schema for `/transfer/originator/create`
    pub fn new(company_name: String) -> TransferOriginatorCreateRequest {
        TransferOriginatorCreateRequest {
            client_id: None,
            secret: None,
            company_name,
        }
    }
}


