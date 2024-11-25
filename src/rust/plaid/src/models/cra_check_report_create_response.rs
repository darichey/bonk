/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraCheckReportCreateResponse : CraCheckReportCreateResponse defines the response schema for `/cra/check_report/create`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CraCheckReportCreateResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl CraCheckReportCreateResponse {
    /// CraCheckReportCreateResponse defines the response schema for `/cra/check_report/create`.
    pub fn new() -> CraCheckReportCreateResponse {
        CraCheckReportCreateResponse {
            request_id: None,
        }
    }
}


