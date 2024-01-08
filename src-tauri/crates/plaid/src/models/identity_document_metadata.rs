/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IdentityDocumentMetadata : In closed beta. Object representing metadata pertaining to the document.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityDocumentMetadata {
    /// The name of the document.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Boolean field indicating if the uploaded document's account number matches the account number we have on file
    #[serde(rename = "is_account_number_match", skip_serializing_if = "Option::is_none")]
    pub is_account_number_match: Option<bool>,
    /// The processing status of the document.  `PROCESSING_COMPLETE`: The document was successfully processed.  `DOCUMENT_ERROR`: The document could not be processed. Possible causes include: The document was an unacceptable document type such as an offer letter or bank statement, the document image was cropped or blurry, or the document was corrupted.  `UNKNOWN` or `null`: An internal error occurred. If this happens repeatedly, contact support or your Plaid account manager.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "uploaded_at", skip_serializing_if = "Option::is_none")]
    pub uploaded_at: Option<String>,
    #[serde(rename = "page_count", skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i32>,
}

impl IdentityDocumentMetadata {
    /// In closed beta. Object representing metadata pertaining to the document.
    pub fn new() -> IdentityDocumentMetadata {
        IdentityDocumentMetadata {
            name: None,
            is_account_number_match: None,
            status: None,
            last_updated: None,
            uploaded_at: None,
            page_count: None,
        }
    }
}


