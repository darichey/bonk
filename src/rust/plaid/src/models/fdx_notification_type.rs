/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FdxNotificationType : Type of Notification

/// Type of Notification
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FdxNotificationType {
    #[serde(rename = "CONSENT_REVOKED")]
    ConsentRevoked,
    #[serde(rename = "CONSENT_UPDATED")]
    ConsentUpdated,
    #[serde(rename = "CUSTOM")]
    Custom,
    #[serde(rename = "SERVICE")]
    Service,
    #[serde(rename = "BALANCE")]
    Balance,
    #[serde(rename = "PLANNED_OUTAGE")]
    PlannedOutage,

}

impl ToString for FdxNotificationType {
    fn to_string(&self) -> String {
        match self {
            Self::ConsentRevoked => String::from("CONSENT_REVOKED"),
            Self::ConsentUpdated => String::from("CONSENT_UPDATED"),
            Self::Custom => String::from("CUSTOM"),
            Self::Service => String::from("SERVICE"),
            Self::Balance => String::from("BALANCE"),
            Self::PlannedOutage => String::from("PLANNED_OUTAGE"),
        }
    }
}

impl Default for FdxNotificationType {
    fn default() -> FdxNotificationType {
        Self::ConsentRevoked
    }
}




