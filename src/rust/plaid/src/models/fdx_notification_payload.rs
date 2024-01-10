/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FdxNotificationPayload : Custom key-value pairs payload for a notification



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FdxNotificationPayload {
    /// ID for the origination entity related to the notification
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "idType", skip_serializing_if = "Option::is_none")]
    pub id_type: Option<crate::models::FdxNotificationPayloadIdType>,
    #[serde(rename = "customFields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<crate::models::FdxfiAttribute>>,
}

impl FdxNotificationPayload {
    /// Custom key-value pairs payload for a notification
    pub fn new() -> FdxNotificationPayload {
        FdxNotificationPayload {
            id: None,
            id_type: None,
            custom_fields: None,
        }
    }
}

