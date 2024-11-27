/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CraCheckReportPartnerInsightsGetResponse : CraPartnerInsightsGetResponse defines the response schema for `/cra/partner_insights/get`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CraCheckReportPartnerInsightsGetResponse {
    #[serde(rename = "report", skip_serializing_if = "Option::is_none")]
    pub report: Option<models::CraPartnerInsights>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl CraCheckReportPartnerInsightsGetResponse {
    /// CraPartnerInsightsGetResponse defines the response schema for `/cra/partner_insights/get`.
    pub fn new(request_id: String) -> CraCheckReportPartnerInsightsGetResponse {
        CraCheckReportPartnerInsightsGetResponse {
            report: None,
            request_id,
        }
    }
}

