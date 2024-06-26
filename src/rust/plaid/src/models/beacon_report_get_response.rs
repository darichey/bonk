/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BeaconReportGetResponse : A Beacon Report describes the type of fraud committed by a user as well as the date the fraud was committed and the total amount of money lost due to the fraud incident.  This information is used to block similar fraud attempts on your platform as well as alert other companies who screen a user with matching identity information. Other companies will not receive any new identity information, just what matched, plus information such as industry, type of fraud, and date of fraud.  You can manage your fraud reports by adding, deleting, or editing reports as you get additional information on fraudulent users.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeaconReportGetResponse {
    /// ID of the associated Beacon Report.
    #[serde(rename = "id")]
    pub id: String,
    /// ID of the associated Beacon User.
    #[serde(rename = "beacon_user_id")]
    pub beacon_user_id: String,
    /// An ISO8601 formatted timestamp.
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::BeaconReportType,
    /// A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(rename = "fraud_date")]
    pub fraud_date: String,
    #[serde(rename = "fraud_amount", deserialize_with = "Option::deserialize")]
    pub fraud_amount: Option<crate::models::FraudAmount>,
    #[serde(rename = "audit_trail")]
    pub audit_trail: crate::models::BeaconAuditTrail,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl BeaconReportGetResponse {
    /// A Beacon Report describes the type of fraud committed by a user as well as the date the fraud was committed and the total amount of money lost due to the fraud incident.  This information is used to block similar fraud attempts on your platform as well as alert other companies who screen a user with matching identity information. Other companies will not receive any new identity information, just what matched, plus information such as industry, type of fraud, and date of fraud.  You can manage your fraud reports by adding, deleting, or editing reports as you get additional information on fraudulent users.
    pub fn new(id: String, beacon_user_id: String, created_at: String, r#type: crate::models::BeaconReportType, fraud_date: String, fraud_amount: Option<crate::models::FraudAmount>, audit_trail: crate::models::BeaconAuditTrail, request_id: String) -> BeaconReportGetResponse {
        BeaconReportGetResponse {
            id,
            beacon_user_id,
            created_at,
            r#type,
            fraud_date,
            fraud_amount,
            audit_trail,
            request_id,
        }
    }
}


