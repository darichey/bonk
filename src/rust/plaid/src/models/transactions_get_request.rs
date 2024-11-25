/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsGetRequest : TransactionsGetRequest defines the request schema for `/transactions/get`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionsGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::TransactionsGetRequestOptions>>,
    /// The access token associated with the Item data is being requested for.
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The earliest date for which data should be returned. Dates should be formatted as YYYY-MM-DD.
    #[serde(rename = "start_date")]
    pub start_date: String,
    /// The latest date for which data should be returned. Dates should be formatted as YYYY-MM-DD.
    #[serde(rename = "end_date")]
    pub end_date: String,
}

impl TransactionsGetRequest {
    /// TransactionsGetRequest defines the request schema for `/transactions/get`
    pub fn new(access_token: String, start_date: String, end_date: String) -> TransactionsGetRequest {
        TransactionsGetRequest {
            client_id: None,
            options: None,
            access_token,
            secret: None,
            start_date,
            end_date,
        }
    }
}


