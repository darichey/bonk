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

/// UserItemsGetResponse : UserItemsGetResponse defines the response schema for `/user/items/get`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserItemsGetResponse {
    #[serde(rename = "items")]
    pub items: Vec<models::Item>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl UserItemsGetResponse {
    /// UserItemsGetResponse defines the response schema for `/user/items/get`
    pub fn new(items: Vec<models::Item>, request_id: String) -> UserItemsGetResponse {
        UserItemsGetResponse {
            items,
            request_id,
        }
    }
}

