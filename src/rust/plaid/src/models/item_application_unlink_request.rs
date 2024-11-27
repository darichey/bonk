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

/// ItemApplicationUnlinkRequest : ItemApplicationUnlinkRequest defines the request schema for `/item/application/unlink/`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemApplicationUnlinkRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The access token associated with the Item data is being requested for.
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// This field will map to the application ID that is returned from /item/application/list, or provided to the institution in an oauth redirect.
    #[serde(rename = "application_id")]
    pub application_id: String,
}

impl ItemApplicationUnlinkRequest {
    /// ItemApplicationUnlinkRequest defines the request schema for `/item/application/unlink/`
    pub fn new(access_token: String, application_id: String) -> ItemApplicationUnlinkRequest {
        ItemApplicationUnlinkRequest {
            client_id: None,
            secret: None,
            access_token,
            application_id,
        }
    }
}

