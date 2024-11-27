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

/// TransferOriginatorGetResponse : Defines the response schema for `/transfer/originator/get`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferOriginatorGetResponse {
    #[serde(rename = "originator")]
    pub originator: models::DetailedOriginator,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransferOriginatorGetResponse {
    /// Defines the response schema for `/transfer/originator/get`
    pub fn new(originator: models::DetailedOriginator, request_id: String) -> TransferOriginatorGetResponse {
        TransferOriginatorGetResponse {
            originator,
            request_id,
        }
    }
}

