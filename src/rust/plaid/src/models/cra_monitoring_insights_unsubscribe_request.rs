/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraMonitoringInsightsUnsubscribeRequest : CraMonitoringInsightsUnsubscribeRequest defines the request schema for `/cra/monitoring_insights/unsubscribe`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CraMonitoringInsightsUnsubscribeRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// A unique identifier for the subscription.
    #[serde(rename = "subscription_id")]
    pub subscription_id: String,
}

impl CraMonitoringInsightsUnsubscribeRequest {
    /// CraMonitoringInsightsUnsubscribeRequest defines the request schema for `/cra/monitoring_insights/unsubscribe`
    pub fn new(subscription_id: String) -> CraMonitoringInsightsUnsubscribeRequest {
        CraMonitoringInsightsUnsubscribeRequest {
            client_id: None,
            secret: None,
            subscription_id,
        }
    }
}


