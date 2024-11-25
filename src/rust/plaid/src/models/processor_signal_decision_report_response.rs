/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProcessorSignalDecisionReportResponse : ProcessorSignalDecisionReportResponse defines the response schema for `/processor/signal/decision/report`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProcessorSignalDecisionReportResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl ProcessorSignalDecisionReportResponse {
    /// ProcessorSignalDecisionReportResponse defines the response schema for `/processor/signal/decision/report`
    pub fn new(request_id: String) -> ProcessorSignalDecisionReportResponse {
        ProcessorSignalDecisionReportResponse {
            request_id,
        }
    }
}


