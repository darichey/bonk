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

/// BeaconUserCreateResponse : A Beacon User represents an end user that has been scanned against the Beacon Network.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeaconUserCreateResponse {
    /// An array of Plaid Item IDs corresponding to the Accounts associated with this Beacon User.
    #[serde(rename = "item_ids")]
    pub item_ids: Vec<String>,
    /// ID of the associated Beacon User.
    #[serde(rename = "id")]
    pub id: String,
    /// The `version` field begins with 1 and increments each time the user is updated.
    #[serde(rename = "version")]
    pub version: i32,
    /// An ISO8601 formatted timestamp.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// An ISO8601 formatted timestamp. This field indicates the last time the resource was modified.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "status")]
    pub status: models::BeaconUserStatus,
    /// ID of the associated Beacon Program.
    #[serde(rename = "program_id")]
    pub program_id: String,
    /// A unique ID that identifies the end user in your system. This ID can also be used to associate user-specific data from other Plaid products. Financial Account Matching requires this field and the `/link/token/create` `client_user_id` to be consistent. Personally identifiable information, such as an email address or phone number, should not be used in the `client_user_id`.
    #[serde(rename = "client_user_id")]
    pub client_user_id: String,
    #[serde(rename = "user")]
    pub user: models::BeaconUserData,
    #[serde(rename = "audit_trail")]
    pub audit_trail: models::BeaconAuditTrail,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl BeaconUserCreateResponse {
    /// A Beacon User represents an end user that has been scanned against the Beacon Network.
    pub fn new(item_ids: Vec<String>, id: String, version: i32, created_at: String, updated_at: String, status: models::BeaconUserStatus, program_id: String, client_user_id: String, user: models::BeaconUserData, audit_trail: models::BeaconAuditTrail, request_id: String) -> BeaconUserCreateResponse {
        BeaconUserCreateResponse {
            item_ids,
            id,
            version,
            created_at,
            updated_at,
            status,
            program_id,
            client_user_id,
            user,
            audit_trail,
            request_id,
        }
    }
}

