/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AuthGetResponse : AuthGetResponse defines the response schema for `/auth/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthGetResponse {
    /// The `accounts` for which numbers are being retrieved.
    #[serde(rename = "accounts")]
    pub accounts: Vec<crate::models::AccountBase>,
    #[serde(rename = "numbers")]
    pub numbers: crate::models::AuthGetNumbers,
    #[serde(rename = "item")]
    pub item: crate::models::Item,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl AuthGetResponse {
    /// AuthGetResponse defines the response schema for `/auth/get`
    pub fn new(accounts: Vec<crate::models::AccountBase>, numbers: crate::models::AuthGetNumbers, item: crate::models::Item, request_id: String) -> AuthGetResponse {
        AuthGetResponse {
            accounts,
            numbers,
            item,
            request_id,
        }
    }
}


