/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkSessionFinishedWebhook : Contains the state of a completed link session, along with the public token if available.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkSessionFinishedWebhook {
    /// `LINK`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `SESSION_FINISHED`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The final status of the link session. Will always be \"SUCCESS\" or \"EXITED\".
    #[serde(rename = "status")]
    pub status: String,
    /// The identifier for the link session.
    #[serde(rename = "link_session_id")]
    pub link_session_id: String,
    /// The link token used to create the link session.
    #[serde(rename = "link_token")]
    pub link_token: String,
    /// The public token generated by the link session.
    #[serde(rename = "public_token", skip_serializing_if = "Option::is_none")]
    pub public_token: Option<String>,
}

impl LinkSessionFinishedWebhook {
    /// Contains the state of a completed link session, along with the public token if available.
    pub fn new(webhook_type: String, webhook_code: String, status: String, link_session_id: String, link_token: String) -> LinkSessionFinishedWebhook {
        LinkSessionFinishedWebhook {
            webhook_type,
            webhook_code,
            status,
            link_session_id,
            link_token,
            public_token: None,
        }
    }
}


