/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraPartnerInsightsGetResponse : CraPartnerInsightsGetResponse defines the response schema for `/cra/partner_insights/get`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CraPartnerInsightsGetResponse {
    #[serde(rename = "report", skip_serializing_if = "Option::is_none")]
    pub report: Option<Vec<crate::models::CraPartnerInsights>>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl CraPartnerInsightsGetResponse {
    /// CraPartnerInsightsGetResponse defines the response schema for `/cra/partner_insights/get`.
    pub fn new(request_id: String) -> CraPartnerInsightsGetResponse {
        CraPartnerInsightsGetResponse {
            report: None,
            request_id,
        }
    }
}


