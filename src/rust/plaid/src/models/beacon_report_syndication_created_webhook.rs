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

/// BeaconReportSyndicationCreatedWebhook : Fired when a report created on the Beacon Network matches with one of your Beacon Users.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeaconReportSyndicationCreatedWebhook {
    /// `BEACON`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `REPORT_SYNDICATION_CREATED`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The ID of the associated Beacon Report Syndication.
    #[serde(rename = "beacon_report_syndication_id")]
    pub beacon_report_syndication_id: String,
    #[serde(rename = "environment")]
    pub environment: models::WebhookEnvironmentValues,
}

impl BeaconReportSyndicationCreatedWebhook {
    /// Fired when a report created on the Beacon Network matches with one of your Beacon Users.
    pub fn new(webhook_type: String, webhook_code: String, beacon_report_syndication_id: String, environment: models::WebhookEnvironmentValues) -> BeaconReportSyndicationCreatedWebhook {
        BeaconReportSyndicationCreatedWebhook {
            webhook_type,
            webhook_code,
            beacon_report_syndication_id,
            environment,
        }
    }
}

