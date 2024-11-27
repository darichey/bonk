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

/// AuthMetadata : Metadata that captures information about the Auth features of an institution.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthMetadata {
    #[serde(rename = "supported_methods", deserialize_with = "Option::deserialize")]
    pub supported_methods: Option<models::AuthSupportedMethods>,
}

impl AuthMetadata {
    /// Metadata that captures information about the Auth features of an institution.
    pub fn new(supported_methods: Option<models::AuthSupportedMethods>) -> AuthMetadata {
        AuthMetadata {
            supported_methods,
        }
    }
}

