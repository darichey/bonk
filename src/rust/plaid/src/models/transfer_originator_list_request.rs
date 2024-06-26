/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferOriginatorListRequest : Defines the request schema for `/transfer/originator/list`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferOriginatorListRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The maximum number of originators to return.
    #[serde(rename = "count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub count: Option<Option<i32>>,
    /// The number of originators to skip before returning results.
    #[serde(rename = "offset", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub offset: Option<Option<i32>>,
}

impl TransferOriginatorListRequest {
    /// Defines the request schema for `/transfer/originator/list`
    pub fn new() -> TransferOriginatorListRequest {
        TransferOriginatorListRequest {
            client_id: None,
            secret: None,
            count: None,
            offset: None,
        }
    }
}


