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

/// SandboxTransferTestClockCreateRequest : Defines the request schema for `/sandbox/transfer/test_clock/create`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SandboxTransferTestClockCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The virtual timestamp on the test clock. If not provided, the current timestamp will be used. This will be of the form `2006-01-02T15:04:05Z`.
    #[serde(rename = "virtual_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub virtual_time: Option<Option<String>>,
}

impl SandboxTransferTestClockCreateRequest {
    /// Defines the request schema for `/sandbox/transfer/test_clock/create`
    pub fn new() -> SandboxTransferTestClockCreateRequest {
        SandboxTransferTestClockCreateRequest {
            client_id: None,
            secret: None,
            virtual_time: None,
        }
    }
}

