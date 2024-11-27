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

/// LinkTokenCreateIdentity : Identity object used to specify document upload
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokenCreateIdentity {
    /// Used to specify whether the Link session is Identity Document Upload
    #[serde(rename = "is_document_upload", skip_serializing_if = "Option::is_none")]
    pub is_document_upload: Option<bool>,
    /// An array of `account_ids`. Currently can only contain one `account_id`. Must be populated if using Document Upload.
    #[serde(rename = "account_ids", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// An array of parsing configurations. Valid parsing configurations are `ocr` and `risk_signals`. If parsing configurations are omitted, defaults to `ocr`
    #[serde(rename = "parsing_configs", skip_serializing_if = "Option::is_none")]
    pub parsing_configs: Option<Vec<models::IncomeVerificationDocParsingConfig>>,
}

impl LinkTokenCreateIdentity {
    /// Identity object used to specify document upload
    pub fn new() -> LinkTokenCreateIdentity {
        LinkTokenCreateIdentity {
            is_document_upload: None,
            account_ids: None,
            parsing_configs: None,
        }
    }
}

