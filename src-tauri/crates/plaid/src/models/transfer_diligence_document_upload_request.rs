/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferDiligenceDocumentUploadRequest : Defines the request schema for `/transfer/diligence/document/upload`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferDiligenceDocumentUploadRequest {
    /// The Client ID of the originator whose document that you want to upload.
    #[serde(rename = "originator_client_id")]
    pub originator_client_id: String,
    /// A file to upload. The file size must be less than 20MB. Supported file extensions: .pdf.
    #[serde(rename = "file")]
    pub file: std::path::PathBuf,
    #[serde(rename = "purpose")]
    pub purpose: crate::models::TransferDocumentPurpose,
}

impl TransferDiligenceDocumentUploadRequest {
    /// Defines the request schema for `/transfer/diligence/document/upload`
    pub fn new(originator_client_id: String, file: std::path::PathBuf, purpose: crate::models::TransferDocumentPurpose) -> TransferDiligenceDocumentUploadRequest {
        TransferDiligenceDocumentUploadRequest {
            originator_client_id,
            file,
            purpose,
        }
    }
}


