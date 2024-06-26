/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CauseAllOf {
    /// The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(rename = "item_id", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
}

impl CauseAllOf {
    pub fn new() -> CauseAllOf {
        CauseAllOf {
            item_id: None,
        }
    }
}


