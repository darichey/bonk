/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InvestmentsRefreshResponse : InvestmentsRefreshResponse defines the response schema for `/investments/refresh`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvestmentsRefreshResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl InvestmentsRefreshResponse {
    /// InvestmentsRefreshResponse defines the response schema for `/investments/refresh`
    pub fn new(request_id: String) -> InvestmentsRefreshResponse {
        InvestmentsRefreshResponse {
            request_id,
        }
    }
}


