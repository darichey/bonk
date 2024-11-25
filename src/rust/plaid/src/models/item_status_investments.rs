/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemStatusInvestments : Information about the last successful and failed investments update for the Item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemStatusInvestments {
    /// [ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of the last successful investments update for the Item. The status will update each time Plaid successfully connects with the institution, regardless of whether any new data is available in the update.
    #[serde(rename = "last_successful_update", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_successful_update: Option<Option<String>>,
    /// [ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of the last failed investments update for the Item. The status will update each time Plaid fails an attempt to connect with the institution, regardless of whether any new data is available in the update.
    #[serde(rename = "last_failed_update", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_failed_update: Option<Option<String>>,
}

impl ItemStatusInvestments {
    /// Information about the last successful and failed investments update for the Item.
    pub fn new() -> ItemStatusInvestments {
        ItemStatusInvestments {
            last_successful_update: None,
            last_failed_update: None,
        }
    }
}


