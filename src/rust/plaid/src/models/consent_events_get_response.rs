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

/// ConsentEventsGetResponse : Describes a historical log of item consent events.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsentEventsGetResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
    /// A list of consent events.
    #[serde(rename = "consent_events")]
    pub consent_events: Vec<models::ConsentEvent>,
}

impl ConsentEventsGetResponse {
    /// Describes a historical log of item consent events.
    pub fn new(request_id: String, consent_events: Vec<models::ConsentEvent>) -> ConsentEventsGetResponse {
        ConsentEventsGetResponse {
            request_id,
            consent_events,
        }
    }
}

