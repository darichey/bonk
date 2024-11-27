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

/// BeaconMatchSummaryAnalysis : Analysis of which fields matched between one Beacon User and another.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeaconMatchSummaryAnalysis {
    #[serde(rename = "address")]
    pub address: models::BeaconMatchSummaryCode,
    #[serde(rename = "date_of_birth")]
    pub date_of_birth: models::BeaconMatchSummaryCode,
    #[serde(rename = "email_address")]
    pub email_address: models::BeaconMatchSummaryCode,
    #[serde(rename = "name")]
    pub name: models::BeaconMatchSummaryCode,
    #[serde(rename = "id_number")]
    pub id_number: models::BeaconMatchSummaryCode,
    #[serde(rename = "ip_address")]
    pub ip_address: models::BeaconMatchSummaryCode,
    #[serde(rename = "phone_number")]
    pub phone_number: models::BeaconMatchSummaryCode,
}

impl BeaconMatchSummaryAnalysis {
    /// Analysis of which fields matched between one Beacon User and another.
    pub fn new(address: models::BeaconMatchSummaryCode, date_of_birth: models::BeaconMatchSummaryCode, email_address: models::BeaconMatchSummaryCode, name: models::BeaconMatchSummaryCode, id_number: models::BeaconMatchSummaryCode, ip_address: models::BeaconMatchSummaryCode, phone_number: models::BeaconMatchSummaryCode) -> BeaconMatchSummaryAnalysis {
        BeaconMatchSummaryAnalysis {
            address,
            date_of_birth,
            email_address,
            name,
            id_number,
            ip_address,
            phone_number,
        }
    }
}

