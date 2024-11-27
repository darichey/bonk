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

/// SandboxUserResetLoginResponse : SandboxUserResetLoginResponse defines the response schema for `/sandbox/user/reset_login`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SandboxUserResetLoginResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl SandboxUserResetLoginResponse {
    /// SandboxUserResetLoginResponse defines the response schema for `/sandbox/user/reset_login`
    pub fn new(request_id: String) -> SandboxUserResetLoginResponse {
        SandboxUserResetLoginResponse {
            request_id,
        }
    }
}

