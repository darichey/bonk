/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BeaconReportSyndicationGetResponse : A Beacon Report Syndication represents a Beacon Report created either by your organization or another Beacon customer that matches a specific Beacon User you've created.  The `analysis` field in the response indicates which fields matched between the originally reported Beacon User and the Beacon User that the report was syndicated to.  The `report` field in the response contains a subset of information from the original report.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeaconReportSyndicationGetResponse {
    /// ID of the associated Beacon Report Syndication.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "report")]
    pub report: crate::models::BeaconReportSyndicationOriginalReport,
    #[serde(rename = "analysis")]
    pub analysis: crate::models::BeaconReportSyndicationAnalysis,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl BeaconReportSyndicationGetResponse {
    /// A Beacon Report Syndication represents a Beacon Report created either by your organization or another Beacon customer that matches a specific Beacon User you've created.  The `analysis` field in the response indicates which fields matched between the originally reported Beacon User and the Beacon User that the report was syndicated to.  The `report` field in the response contains a subset of information from the original report.
    pub fn new(id: String, report: crate::models::BeaconReportSyndicationOriginalReport, analysis: crate::models::BeaconReportSyndicationAnalysis, request_id: String) -> BeaconReportSyndicationGetResponse {
        BeaconReportSyndicationGetResponse {
            id,
            report,
            analysis,
            request_id,
        }
    }
}


