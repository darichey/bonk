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

/// LinkSessionSuccessMetadata : Displayed once a user has successfully linked their Item.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkSessionSuccessMetadata {
    #[serde(rename = "institution", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub institution: Option<Option<Box<models::LinkSessionSuccessMetadataInstitution>>>,
    /// A list of accounts attached to the connected Item. If Account Select is enabled via the developer dashboard, `accounts` will only include selected accounts.
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<models::LinkSessionSuccessMetadataAccount>>,
    /// A unique identifier associated with a user's actions and events through the Link flow. Include this identifier when opening a support ticket for faster turnaround.
    #[serde(rename = "link_session_id", skip_serializing_if = "Option::is_none")]
    pub link_session_id: Option<String>,
    #[serde(rename = "transfer_status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub transfer_status: Option<Option<models::LinkSessionSuccessMetadataTransferStatus>>,
}

impl LinkSessionSuccessMetadata {
    /// Displayed once a user has successfully linked their Item.
    pub fn new() -> LinkSessionSuccessMetadata {
        LinkSessionSuccessMetadata {
            institution: None,
            accounts: None,
            link_session_id: None,
            transfer_status: None,
        }
    }
}

