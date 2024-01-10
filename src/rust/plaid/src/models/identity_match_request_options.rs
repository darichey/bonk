/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IdentityMatchRequestOptions : An optional object to filter /identity/match results



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityMatchRequestOptions {
    /// An array of `account_ids` to perform fuzzy match
    #[serde(rename = "account_ids", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

impl IdentityMatchRequestOptions {
    /// An optional object to filter /identity/match results
    pub fn new() -> IdentityMatchRequestOptions {
        IdentityMatchRequestOptions {
            account_ids: None,
        }
    }
}

