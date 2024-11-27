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

/// ItemImportResponse : ItemImportResponse defines the response schema for `/item/import`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemImportResponse {
    /// The access token associated with the Item data is being requested for.
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl ItemImportResponse {
    /// ItemImportResponse defines the response schema for `/item/import`
    pub fn new(access_token: String, request_id: String) -> ItemImportResponse {
        ItemImportResponse {
            access_token,
            request_id,
        }
    }
}

