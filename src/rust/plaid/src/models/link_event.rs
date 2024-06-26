/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkEvent : An event that occurred while the user was going through Link



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkEvent {
    /// Event name
    #[serde(rename = "event_name")]
    pub event_name: String,
    /// Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// UUID that can be used to deduplicate events
    #[serde(rename = "event_id")]
    pub event_id: String,
}

impl LinkEvent {
    /// An event that occurred while the user was going through Link
    pub fn new(event_name: String, timestamp: String, event_id: String) -> LinkEvent {
        LinkEvent {
            event_name,
            timestamp,
            event_id,
        }
    }
}


