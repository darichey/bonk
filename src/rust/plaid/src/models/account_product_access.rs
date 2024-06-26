/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AccountProductAccess : Allow the application to access specific products on this account



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountProductAccess {
    /// Allow the application to access account data. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    #[serde(rename = "account_data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_data: Option<Option<bool>>,
    /// Allow the application to access bank statements. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    #[serde(rename = "statements", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub statements: Option<Option<bool>>,
    /// Allow the application to access tax documents. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    #[serde(rename = "tax_documents", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tax_documents: Option<Option<bool>>,
}

impl AccountProductAccess {
    /// Allow the application to access specific products on this account
    pub fn new() -> AccountProductAccess {
        AccountProductAccess {
            account_data: None,
            statements: None,
            tax_documents: None,
        }
    }
}


