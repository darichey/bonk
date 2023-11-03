/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AssetReportItem : A representation of an Item within an Asset Report.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetReportItem {
    /// The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The full financial institution name associated with the Item.
    #[serde(rename = "institution_name")]
    pub institution_name: String,
    /// The id of the financial institution associated with the Item.
    #[serde(rename = "institution_id")]
    pub institution_id: String,
    /// The date and time when this Item’s data was last retrieved from the financial institution, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    #[serde(rename = "date_last_updated")]
    pub date_last_updated: String,
    /// Data about each of the accounts open on the Item.
    #[serde(rename = "accounts")]
    pub accounts: Vec<crate::models::AccountAssets>,
}

impl AssetReportItem {
    /// A representation of an Item within an Asset Report.
    pub fn new(item_id: String, institution_name: String, institution_id: String, date_last_updated: String, accounts: Vec<crate::models::AccountAssets>) -> AssetReportItem {
        AssetReportItem {
            item_id,
            institution_name,
            institution_id,
            date_last_updated,
            accounts,
        }
    }
}


