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

/// DepositSwitchTokenCreateResponse : (Deprecated) DepositSwitchTokenCreateResponse defines the response schema for `/deposit_switch/token/create`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DepositSwitchTokenCreateResponse {
    /// Deposit switch token, used to initialize Link for the Deposit Switch product
    #[serde(rename = "deposit_switch_token")]
    pub deposit_switch_token: String,
    /// Expiration time of the token, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format
    #[serde(rename = "deposit_switch_token_expiration_time")]
    pub deposit_switch_token_expiration_time: String,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl DepositSwitchTokenCreateResponse {
    /// (Deprecated) DepositSwitchTokenCreateResponse defines the response schema for `/deposit_switch/token/create`
    pub fn new(deposit_switch_token: String, deposit_switch_token_expiration_time: String, request_id: String) -> DepositSwitchTokenCreateResponse {
        DepositSwitchTokenCreateResponse {
            deposit_switch_token,
            deposit_switch_token_expiration_time,
            request_id,
        }
    }
}

