/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FdxNotification : Provides the base fields of a notification. Clients will read the `type` property to determine the expected notification payload



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FdxNotification {
    /// Id of notification
    #[serde(rename = "notificationId")]
    pub notification_id: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::FdxNotificationType,
    /// ISO 8601 date-time in format 'YYYY-MM-DDThh:mm:ss.nnn[Z|[+|-]hh:mm]' according to [IETF RFC3339](https://xml2rfc.tools.ietf.org/public/rfc/html/rfc3339.html#anchor14)
    #[serde(rename = "sentOn")]
    pub sent_on: String,
    #[serde(rename = "category")]
    pub category: crate::models::FdxNotificationCategory,
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<crate::models::FdxNotificationSeverity>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<crate::models::FdxNotificationPriority>,
    #[serde(rename = "publisher", skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Box<crate::models::FdxParty>>,
    #[serde(rename = "subscriber", skip_serializing_if = "Option::is_none")]
    pub subscriber: Option<Box<crate::models::FdxParty>>,
    #[serde(rename = "notificationPayload")]
    pub notification_payload: Box<crate::models::FdxNotificationPayload>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<Box<crate::models::FdxHateoasLink>>,
}

impl FdxNotification {
    /// Provides the base fields of a notification. Clients will read the `type` property to determine the expected notification payload
    pub fn new(notification_id: String, r#type: crate::models::FdxNotificationType, sent_on: String, category: crate::models::FdxNotificationCategory, notification_payload: crate::models::FdxNotificationPayload) -> FdxNotification {
        FdxNotification {
            notification_id,
            r#type,
            sent_on,
            category,
            severity: None,
            priority: None,
            publisher: None,
            subscriber: None,
            notification_payload: Box::new(notification_payload),
            url: None,
        }
    }
}


