/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsUserInsightsGetRequest : TransactionsUserInsightsGetRequest defines the request schema for `/beta/transactions/user_insights/v1/get`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionsUserInsightsGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// A unique client-provided user_id to retrieve insights for.
    #[serde(rename = "client_user_id")]
    pub client_user_id: String,
}

impl TransactionsUserInsightsGetRequest {
    /// TransactionsUserInsightsGetRequest defines the request schema for `/beta/transactions/user_insights/v1/get`.
    pub fn new(client_user_id: String) -> TransactionsUserInsightsGetRequest {
        TransactionsUserInsightsGetRequest {
            client_id: None,
            secret: None,
            client_user_id,
        }
    }
}


