/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IdentityDocumentUploadMetadata : Metadata pertaining to the document.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityDocumentUploadMetadata {
    /// The submitted document type. Currently, this will always be `BANK_STATEMENT`.
    #[serde(rename = "document_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub document_type: Option<Option<String>>,
    /// Boolean field indicating whether the uploaded document's account number matches the account number we have on file. If `false`, it is not recommended to accept the uploaded identity data as accurate without further verification.
    #[serde(rename = "is_account_number_match", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_account_number_match: Option<Option<bool>>,
    /// The number of pages in the uploaded document.
    #[serde(rename = "page_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub page_count: Option<Option<i32>>,
    /// The timestamp when the document was last updated.
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// The timestamp when the document was originally uploaded.
    #[serde(rename = "uploaded_at", skip_serializing_if = "Option::is_none")]
    pub uploaded_at: Option<String>,
}

impl IdentityDocumentUploadMetadata {
    /// Metadata pertaining to the document.
    pub fn new() -> IdentityDocumentUploadMetadata {
        IdentityDocumentUploadMetadata {
            document_type: None,
            is_account_number_match: None,
            page_count: None,
            last_updated: None,
            uploaded_at: None,
        }
    }
}

