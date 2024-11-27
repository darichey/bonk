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

/// InstitutionsGetByIdResponse : InstitutionsGetByIdResponse defines the response schema for `/institutions/get_by_id`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstitutionsGetByIdResponse {
    #[serde(rename = "institution")]
    pub institution: models::Institution,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl InstitutionsGetByIdResponse {
    /// InstitutionsGetByIdResponse defines the response schema for `/institutions/get_by_id`
    pub fn new(institution: models::Institution, request_id: String) -> InstitutionsGetByIdResponse {
        InstitutionsGetByIdResponse {
            institution,
            request_id,
        }
    }
}

