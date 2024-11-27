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

/// SandboxProcessorTokenCreateRequestOptions : An optional set of options to be used when configuring the Item. If specified, must not be `null`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SandboxProcessorTokenCreateRequestOptions {
    /// Test username to use for the creation of the Sandbox Item. Default value is `user_good`.
    #[serde(rename = "override_username", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub override_username: Option<Option<String>>,
    /// Test password to use for the creation of the Sandbox Item. Default value is `pass_good`.
    #[serde(rename = "override_password", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub override_password: Option<Option<String>>,
}

impl SandboxProcessorTokenCreateRequestOptions {
    /// An optional set of options to be used when configuring the Item. If specified, must not be `null`.
    pub fn new() -> SandboxProcessorTokenCreateRequestOptions {
        SandboxProcessorTokenCreateRequestOptions {
            override_username: None,
            override_password: None,
        }
    }
}

