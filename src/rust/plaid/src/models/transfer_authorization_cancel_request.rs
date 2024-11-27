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

/// TransferAuthorizationCancelRequest : Defines the request schema for `/transfer/authorization/cancel`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferAuthorizationCancelRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Plaid’s unique identifier for a transfer authorization.
    #[serde(rename = "authorization_id")]
    pub authorization_id: String,
}

impl TransferAuthorizationCancelRequest {
    /// Defines the request schema for `/transfer/authorization/cancel`
    pub fn new(authorization_id: String) -> TransferAuthorizationCancelRequest {
        TransferAuthorizationCancelRequest {
            client_id: None,
            secret: None,
            authorization_id,
        }
    }
}

