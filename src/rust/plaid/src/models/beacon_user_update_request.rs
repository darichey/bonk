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

/// BeaconUserUpdateRequest : Request input for updating the identity data of a Beacon User.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeaconUserUpdateRequest {
    /// ID of the associated Beacon User.
    #[serde(rename = "beacon_user_id")]
    pub beacon_user_id: String,
    #[serde(rename = "user", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user: Option<Option<models::BeaconUserUpdateRequestData>>,
    /// Send this array of access tokens to add accounts to this user for evaluation. This will add accounts to this Beacon User. If left null only existing accounts will be returned in response. A maximum of 50 accounts total can be added to a Beacon User.
    #[serde(rename = "access_tokens", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub access_tokens: Option<Option<Vec<String>>>,
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl BeaconUserUpdateRequest {
    /// Request input for updating the identity data of a Beacon User.
    pub fn new(beacon_user_id: String) -> BeaconUserUpdateRequest {
        BeaconUserUpdateRequest {
            beacon_user_id,
            user: None,
            access_tokens: None,
            client_id: None,
            secret: None,
        }
    }
}

