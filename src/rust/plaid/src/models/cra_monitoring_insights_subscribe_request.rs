/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraMonitoringInsightsSubscribeRequest : CraMonitoringInsightsSubscribeRequest defines the request schema for `/cra/monitoring_insights/subscribe`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CraMonitoringInsightsSubscribeRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The user token associated with the User data is being requested for.
    #[serde(rename = "user_token")]
    pub user_token: String,
    /// URL to which Plaid will send Monitoring Insights webhooks, for example when the requested Monitoring Insights is ready.
    #[serde(rename = "webhook")]
    pub webhook: String,
}

impl CraMonitoringInsightsSubscribeRequest {
    /// CraMonitoringInsightsSubscribeRequest defines the request schema for `/cra/monitoring_insights/subscribe`
    pub fn new(user_token: String, webhook: String) -> CraMonitoringInsightsSubscribeRequest {
        CraMonitoringInsightsSubscribeRequest {
            client_id: None,
            secret: None,
            user_token,
            webhook,
        }
    }
}


