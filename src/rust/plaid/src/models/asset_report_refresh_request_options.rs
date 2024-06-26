/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AssetReportRefreshRequestOptions : An optional object to filter `/asset_report/refresh` results. If provided, cannot be `null`. If not specified, the `options` from the original call to `/asset_report/create` will be used.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetReportRefreshRequestOptions {
    /// Client-generated identifier, which can be used by lenders to track loan applications.
    #[serde(rename = "client_report_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub client_report_id: Option<Option<String>>,
    /// URL to which Plaid will send Assets webhooks, for example when the requested Asset Report is ready.
    #[serde(rename = "webhook", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Option<String>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::models::AssetReportUser>,
}

impl AssetReportRefreshRequestOptions {
    /// An optional object to filter `/asset_report/refresh` results. If provided, cannot be `null`. If not specified, the `options` from the original call to `/asset_report/create` will be used.
    pub fn new() -> AssetReportRefreshRequestOptions {
        AssetReportRefreshRequestOptions {
            client_report_id: None,
            webhook: None,
            user: None,
        }
    }
}


