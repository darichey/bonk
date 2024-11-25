/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IssuesSearchResponse : IssuesSearchResponse defines the response schema for `/issues/search`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IssuesSearchResponse {
    /// A list of issues affecting the Item, session, or request passed in, conforming to the Issues data model. An empty list indicates that no matching issues were found.
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<crate::models::Issue>>,
    /// A unique identifier for the API request.
    #[serde(rename = "request_id", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl IssuesSearchResponse {
    /// IssuesSearchResponse defines the response schema for `/issues/search`.
    pub fn new() -> IssuesSearchResponse {
        IssuesSearchResponse {
            issues: None,
            request_id: None,
        }
    }
}


