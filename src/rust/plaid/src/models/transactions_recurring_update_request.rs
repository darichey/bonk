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

/// TransactionsRecurringUpdateRequest : TransactionsRecurringUpdateRequest defined the request schema for `/transactions/recurring/streams/update` endpoint.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionsRecurringUpdateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// The access token associated with the Item data is being requested for.
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// A list of all the operations to be performed. This will either all succeed or all fail.
    #[serde(rename = "inputs")]
    pub inputs: Vec<models::TransactionsRecurringUpdateInput>,
}

impl TransactionsRecurringUpdateRequest {
    /// TransactionsRecurringUpdateRequest defined the request schema for `/transactions/recurring/streams/update` endpoint.
    pub fn new(access_token: String, inputs: Vec<models::TransactionsRecurringUpdateInput>) -> TransactionsRecurringUpdateRequest {
        TransactionsRecurringUpdateRequest {
            client_id: None,
            access_token,
            secret: None,
            inputs,
        }
    }
}

