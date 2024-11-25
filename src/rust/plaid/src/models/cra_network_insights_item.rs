/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraNetworkInsightsItem : Contains data about the connected Item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CraNetworkInsightsItem {
    /// The ID for the institution the user linked.
    #[serde(rename = "institution_id")]
    pub institution_id: String,
    /// The name of the institution the user linked.
    #[serde(rename = "institution_name")]
    pub institution_name: String,
    /// The identifier for the Item.
    #[serde(rename = "item_id")]
    pub item_id: String,
}

impl CraNetworkInsightsItem {
    /// Contains data about the connected Item.
    pub fn new(institution_id: String, institution_name: String, item_id: String) -> CraNetworkInsightsItem {
        CraNetworkInsightsItem {
            institution_id,
            institution_name,
            item_id,
        }
    }
}


