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

/// BeaconReportSyndicationOriginalReport : A subset of information from a Beacon Report that has been syndicated to a matching Beacon User in your program.  The `id` field in the response is the ID of the original report that was syndicated. If the original report was created by your organization, the field will be filled with the ID of the report. Otherwise, the field will be `null` indicating that the original report was created by another Beacon customer.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeaconReportSyndicationOriginalReport {
    /// ID of the associated Beacon Report.
    #[serde(rename = "id", deserialize_with = "Option::deserialize")]
    pub id: Option<String>,
    /// An ISO8601 formatted timestamp.
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "type")]
    pub r#type: models::BeaconReportType,
    /// A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(rename = "fraud_date", deserialize_with = "Option::deserialize")]
    pub fraud_date: Option<String>,
    /// A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(rename = "event_date")]
    pub event_date: String,
}

impl BeaconReportSyndicationOriginalReport {
    /// A subset of information from a Beacon Report that has been syndicated to a matching Beacon User in your program.  The `id` field in the response is the ID of the original report that was syndicated. If the original report was created by your organization, the field will be filled with the ID of the report. Otherwise, the field will be `null` indicating that the original report was created by another Beacon customer.
    pub fn new(id: Option<String>, created_at: String, r#type: models::BeaconReportType, fraud_date: Option<String>, event_date: String) -> BeaconReportSyndicationOriginalReport {
        BeaconReportSyndicationOriginalReport {
            id,
            created_at,
            r#type,
            fraud_date,
            event_date,
        }
    }
}

