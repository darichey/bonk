/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditSessionItemAddResult : The details of an Item add in Link.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditSessionItemAddResult {
    /// Returned once a user has successfully linked their Item.
    #[serde(rename = "public_token", skip_serializing_if = "Option::is_none")]
    pub public_token: Option<String>,
    /// The Plaid Item ID. The `item_id` is always unique; linking the same account at the same institution twice will result in two Items with different `item_id` values. Like all Plaid identifiers, the `item_id` is case-sensitive.
    #[serde(rename = "item_id", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    /// The Plaid Institution ID associated with the Item.
    #[serde(rename = "institution_id", skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
}

impl CreditSessionItemAddResult {
    /// The details of an Item add in Link.
    pub fn new() -> CreditSessionItemAddResult {
        CreditSessionItemAddResult {
            public_token: None,
            item_id: None,
            institution_id: None,
        }
    }
}


