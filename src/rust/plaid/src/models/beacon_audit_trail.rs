/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BeaconAuditTrail : Information about the last change made to the parent object specifying what caused the change as well as when it occurred.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeaconAuditTrail {
    #[serde(rename = "source")]
    pub source: crate::models::BeaconAuditTrailSource,
    /// ID of the associated user.
    #[serde(rename = "dashboard_user_id", deserialize_with = "Option::deserialize")]
    pub dashboard_user_id: Option<String>,
}

impl BeaconAuditTrail {
    /// Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub fn new(source: crate::models::BeaconAuditTrailSource, dashboard_user_id: Option<String>) -> BeaconAuditTrail {
        BeaconAuditTrail {
            source,
            dashboard_user_id,
        }
    }
}


