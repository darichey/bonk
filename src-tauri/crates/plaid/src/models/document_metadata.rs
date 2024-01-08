/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DocumentMetadata : An object representing metadata from the end user's uploaded document.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentMetadata {
    /// The name of the document.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The processing status of the document.  `PROCESSING_COMPLETE`: The document was successfully processed.  `DOCUMENT_ERROR`: The document could not be processed. Possible causes include: The document was an unacceptable document type such as an offer letter or bank statement, the document image was cropped or blurry, or the document was corrupted.  `UNKNOWN` or `null`: An internal error occurred. If this happens repeatedly, contact support or your Plaid account manager.
    #[serde(rename = "status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<String>>,
    /// An identifier of the document that is also present in the paystub response.
    #[serde(rename = "doc_id", skip_serializing_if = "Option::is_none")]
    pub doc_id: Option<String>,
    #[serde(rename = "doc_type", skip_serializing_if = "Option::is_none")]
    pub doc_type: Option<crate::models::DocType>,
}

impl DocumentMetadata {
    /// An object representing metadata from the end user's uploaded document.
    pub fn new() -> DocumentMetadata {
        DocumentMetadata {
            name: None,
            status: None,
            doc_id: None,
            doc_type: None,
        }
    }
}


