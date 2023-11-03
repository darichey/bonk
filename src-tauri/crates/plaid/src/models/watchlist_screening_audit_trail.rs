/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WatchlistScreeningAuditTrail : Information about the last change made to the parent object specifying what caused the change as well as when it occurred.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WatchlistScreeningAuditTrail {
    #[serde(rename = "source")]
    pub source: crate::models::Source,
    /// ID of the associated user.
    #[serde(rename = "dashboard_user_id", deserialize_with = "Option::deserialize")]
    pub dashboard_user_id: Option<String>,
    /// An ISO8601 formatted timestamp.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}

impl WatchlistScreeningAuditTrail {
    /// Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub fn new(source: crate::models::Source, dashboard_user_id: Option<String>, timestamp: String) -> WatchlistScreeningAuditTrail {
        WatchlistScreeningAuditTrail {
            source,
            dashboard_user_id,
            timestamp,
        }
    }
}


