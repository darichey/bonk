/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AssetReportGetRequestOptions : An optional object to filter or add data to `/asset_report/get` results. If provided, must be non-`null`.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetReportGetRequestOptions {
    /// The maximum number of days of history to include in the Asset Report.
    #[serde(rename = "days_to_include", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub days_to_include: Option<Option<i32>>,
}

impl AssetReportGetRequestOptions {
    /// An optional object to filter or add data to `/asset_report/get` results. If provided, must be non-`null`.
    pub fn new() -> AssetReportGetRequestOptions {
        AssetReportGetRequestOptions {
            days_to_include: None,
        }
    }
}


