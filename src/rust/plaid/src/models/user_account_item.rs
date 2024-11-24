/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserAccountItem : An Item created during a Layer authorization session.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserAccountItem {
    /// The Plaid Item ID. The `item_id` is always unique; linking the same account at the same institution twice will result in two Items with different `item_id` values. Like all Plaid identifiers, the `item_id` is case-sensitive.
    #[serde(rename = "item_id", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    /// The access token associated with the Item data is being requested for.
    #[serde(rename = "access_token", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}

impl UserAccountItem {
    /// An Item created during a Layer authorization session.
    pub fn new() -> UserAccountItem {
        UserAccountItem {
            item_id: None,
            access_token: None,
        }
    }
}


