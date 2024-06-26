/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProcessorSignalEvaluateResponse : ProcessorSignalEvaluateResponse defines the response schema for `/processor/signal/evaluate`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessorSignalEvaluateResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
    #[serde(rename = "scores")]
    pub scores: crate::models::SignalScores,
    #[serde(rename = "core_attributes", skip_serializing_if = "Option::is_none")]
    pub core_attributes: Option<Box<crate::models::SignalEvaluateCoreAttributes>>,
    /// If bank information was not available to be used in the Signal model, this array contains warnings describing why bank data is missing. If you want to receive an API error instead of Signal scores in the case of missing bank data, file a support ticket or contact your Plaid account manager.
    #[serde(rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<crate::models::SignalWarning>>,
}

impl ProcessorSignalEvaluateResponse {
    /// ProcessorSignalEvaluateResponse defines the response schema for `/processor/signal/evaluate`
    pub fn new(request_id: String, scores: crate::models::SignalScores) -> ProcessorSignalEvaluateResponse {
        ProcessorSignalEvaluateResponse {
            request_id,
            scores,
            core_attributes: None,
            warnings: None,
        }
    }
}


