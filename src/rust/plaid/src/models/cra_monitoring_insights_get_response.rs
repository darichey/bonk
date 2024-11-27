/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraMonitoringInsightsGetResponse : CraMonitoringInsightsGetResponse defines the response schema for `cra/monitoring_insights/get`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CraMonitoringInsightsGetResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
    /// An array of Monitoring Insights Items associated with the user.
    #[serde(rename = "items")]
    pub items: Vec<crate::models::CraMonitoringInsightsItem>,
}

impl CraMonitoringInsightsGetResponse {
    /// CraMonitoringInsightsGetResponse defines the response schema for `cra/monitoring_insights/get`
    pub fn new(request_id: String, items: Vec<crate::models::CraMonitoringInsightsItem>) -> CraMonitoringInsightsGetResponse {
        CraMonitoringInsightsGetResponse {
            request_id,
            items,
        }
    }
}

