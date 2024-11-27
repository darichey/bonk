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

/// LinkTokenGetSessionsResponse : An object containing information about a link session. Session data will be provided for up to six hours after the session has ended.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokenGetSessionsResponse {
    /// The unique ID for the link session.
    #[serde(rename = "link_session_id")]
    pub link_session_id: String,
    /// The timestamp at which the link session was first started, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    #[serde(rename = "started_at", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// The timestamp at which the link session was finished, if available, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    #[serde(rename = "finished_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<Option<String>>,
    #[serde(rename = "on_success", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub on_success: Option<Option<Box<models::LinkSessionSuccess>>>,
    #[serde(rename = "on_exit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub on_exit: Option<Option<models::LinkSessionExitDeprecated>>,
    #[serde(rename = "exit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub exit: Option<Option<models::LinkSessionExit>>,
    /// List of customer-related Link events
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<models::LinkEvent>>,
    #[serde(rename = "results", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub results: Option<Option<models::LinkSessionResults>>,
}

impl LinkTokenGetSessionsResponse {
    /// An object containing information about a link session. Session data will be provided for up to six hours after the session has ended.
    pub fn new(link_session_id: String) -> LinkTokenGetSessionsResponse {
        LinkTokenGetSessionsResponse {
            link_session_id,
            started_at: None,
            finished_at: None,
            on_success: None,
            on_exit: None,
            exit: None,
            events: None,
            results: None,
        }
    }
}

