/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkEventsWebhook : This webhook contains a summary of the events from a Link session and will be fired after the user finishes going through Link. If the user abandons the Link flow (i.e., closes the hosted link webpage or leaves Link open for too long without taking any action), the webhook will be fired 5-15 minutes after the last user interaction. A single Link session may occasionally generate multiple `EVENTS` webhooks. If this occurs, the new webhook will contain all previous events for the session, as well as new events that occurred since the previous `EVENTS` webhook was sent. If this occurs, events can be grouped using the `link_session_id` field and, if necessary, de-duplicated using the `event_id` field.  By default, the `EVENTS` webhook is sent only for sessions where the end user goes through a Hosted Link flow (including Link Recovery flows). If you would like to receive this webhook for sessions not using Hosted Link, contact your Account Manager or Support.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkEventsWebhook {
    /// `LINK`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `EVENTS`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The Link events emitted during the Link session
    #[serde(rename = "events")]
    pub events: Vec<crate::models::LinkEvent>,
    /// An identifier for the Link session these events occurred in
    #[serde(rename = "link_session_id")]
    pub link_session_id: String,
    /// The Link token used to create the Link session these events are from
    #[serde(rename = "link_token")]
    pub link_token: String,
}

impl LinkEventsWebhook {
    /// This webhook contains a summary of the events from a Link session and will be fired after the user finishes going through Link. If the user abandons the Link flow (i.e., closes the hosted link webpage or leaves Link open for too long without taking any action), the webhook will be fired 5-15 minutes after the last user interaction. A single Link session may occasionally generate multiple `EVENTS` webhooks. If this occurs, the new webhook will contain all previous events for the session, as well as new events that occurred since the previous `EVENTS` webhook was sent. If this occurs, events can be grouped using the `link_session_id` field and, if necessary, de-duplicated using the `event_id` field.  By default, the `EVENTS` webhook is sent only for sessions where the end user goes through a Hosted Link flow (including Link Recovery flows). If you would like to receive this webhook for sessions not using Hosted Link, contact your Account Manager or Support.
    pub fn new(webhook_type: String, webhook_code: String, events: Vec<crate::models::LinkEvent>, link_session_id: String, link_token: String) -> LinkEventsWebhook {
        LinkEventsWebhook {
            webhook_type,
            webhook_code,
            events,
            link_session_id,
            link_token,
        }
    }
}


