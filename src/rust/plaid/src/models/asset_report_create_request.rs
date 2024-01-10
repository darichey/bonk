/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AssetReportCreateRequest : AssetReportCreateRequest defines the request schema for `/asset_report/create`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetReportCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// An array of access tokens corresponding to the Items that will be included in the report. The `assets` product must have been initialized for the Items during link; the Assets product cannot be added after initialization.
    #[serde(rename = "access_tokens", skip_serializing_if = "Option::is_none")]
    pub access_tokens: Option<Vec<String>>,
    /// The maximum integer number of days of history to include in the Asset Report. If using Fannie Mae Day 1 Certainty, `days_requested` must be at least 61 for new originations or at least 31 for refinancings.  An Asset Report requested with \"Additional History\" (that is, with more than 61 days of transaction history) will incur an Additional History fee.
    #[serde(rename = "days_requested")]
    pub days_requested: i32,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::AssetReportCreateRequestOptions>>,
}

impl AssetReportCreateRequest {
    /// AssetReportCreateRequest defines the request schema for `/asset_report/create`
    pub fn new(days_requested: i32) -> AssetReportCreateRequest {
        AssetReportCreateRequest {
            client_id: None,
            secret: None,
            access_tokens: None,
            days_requested,
            options: None,
        }
    }
}

