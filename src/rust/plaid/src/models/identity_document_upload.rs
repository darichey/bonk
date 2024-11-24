/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IdentityDocumentUpload : Document object with metadata of the uploaded document



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IdentityDocumentUpload {
    /// A UUID identifying the document.
    #[serde(rename = "document_id", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::models::IdentityDocumentUploadMetadata>,
    #[serde(rename = "risk_insights", skip_serializing_if = "Option::is_none")]
    pub risk_insights: Option<crate::models::IdentityDocumentUploadRiskInsights>,
}

impl IdentityDocumentUpload {
    /// Document object with metadata of the uploaded document
    pub fn new() -> IdentityDocumentUpload {
        IdentityDocumentUpload {
            document_id: None,
            metadata: None,
            risk_insights: None,
        }
    }
}


