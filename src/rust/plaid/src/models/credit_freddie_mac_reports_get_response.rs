/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditFreddieMacReportsGetResponse : CreditFreddieMacReportsGetResponse defines the response schema for `/credit/freddie_mac/reports/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditFreddieMacReportsGetResponse {
    #[serde(rename = "DEAL")]
    pub deal: crate::models::CreditFreddieMacVerificationOfAssetsDeal,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
    /// The Verification Of Assets (VOA) schema version.
    #[serde(rename = "SchemaVersion")]
    pub schema_version: f32,
}

impl CreditFreddieMacReportsGetResponse {
    /// CreditFreddieMacReportsGetResponse defines the response schema for `/credit/freddie_mac/reports/get`
    pub fn new(deal: crate::models::CreditFreddieMacVerificationOfAssetsDeal, request_id: String, schema_version: f32) -> CreditFreddieMacReportsGetResponse {
        CreditFreddieMacReportsGetResponse {
            deal,
            request_id,
            schema_version,
        }
    }
}


