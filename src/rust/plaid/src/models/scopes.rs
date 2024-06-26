/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Scopes : The scopes object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Scopes {
    #[serde(rename = "product_access", skip_serializing_if = "Option::is_none")]
    pub product_access: Option<crate::models::ProductAccess>,
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<crate::models::AccountAccess>>,
    /// Allow access to newly opened accounts as they are opened. If unset, defaults to `true`.
    #[serde(rename = "new_accounts", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub new_accounts: Option<Option<bool>>,
}

impl Scopes {
    /// The scopes object
    pub fn new() -> Scopes {
        Scopes {
            product_access: None,
            accounts: None,
            new_accounts: None,
        }
    }
}


