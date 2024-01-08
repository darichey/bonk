/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BeaconReportSyndicationListResponse : The response schema for `/beacon/report_syndication/list`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeaconReportSyndicationListResponse {
    #[serde(rename = "beacon_report_syndications")]
    pub beacon_report_syndications: Vec<crate::models::BeaconReportSyndication>,
    /// An identifier that determines which page of results you receive.
    #[serde(rename = "next_cursor", deserialize_with = "Option::deserialize")]
    pub next_cursor: Option<String>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl BeaconReportSyndicationListResponse {
    /// The response schema for `/beacon/report_syndication/list`
    pub fn new(beacon_report_syndications: Vec<crate::models::BeaconReportSyndication>, next_cursor: Option<String>, request_id: String) -> BeaconReportSyndicationListResponse {
        BeaconReportSyndicationListResponse {
            beacon_report_syndications,
            next_cursor,
            request_id,
        }
    }
}


