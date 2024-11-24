/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NetworkInsightsReport : Contains data for the Network Insights Report.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkInsightsReport {
    /// The unique identifier associated with the Network Insights report object.
    #[serde(rename = "report_id")]
    pub report_id: String,
    /// The time when the Network Insights Report was generated.
    #[serde(rename = "generated_time")]
    pub generated_time: String,
    /// A map of network attributes, where the key is a string, and the value is a float, int, or boolean.
    #[serde(rename = "network_attributes")]
    pub network_attributes: serde_json::Value,
    /// A list of Items associated with the provided access_tokens.
    #[serde(rename = "items")]
    pub items: Vec<crate::models::NetworkInsightsItem>,
}

impl NetworkInsightsReport {
    /// Contains data for the Network Insights Report.
    pub fn new(report_id: String, generated_time: String, network_attributes: serde_json::Value, items: Vec<crate::models::NetworkInsightsItem>) -> NetworkInsightsReport {
        NetworkInsightsReport {
            report_id,
            generated_time,
            network_attributes,
            items,
        }
    }
}


