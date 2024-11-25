/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// StatementsDownloadRequest : StatementsDownloadRequest defines the request schema for `/statements/download`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StatementsDownloadRequest {
    /// The access token associated with the Item data is being requested for.
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Plaid's unique identifier for the statements.
    #[serde(rename = "statement_id")]
    pub statement_id: String,
}

impl StatementsDownloadRequest {
    /// StatementsDownloadRequest defines the request schema for `/statements/download`
    pub fn new(access_token: String, statement_id: String) -> StatementsDownloadRequest {
        StatementsDownloadRequest {
            access_token,
            client_id: None,
            secret: None,
            statement_id,
        }
    }
}


