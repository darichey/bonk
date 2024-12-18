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

/// LinkTokenCreateRequestCraOptions : Specifies options for initializing Link for use with Plaid Check products
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestCraOptions {
    /// The number of days of history to include in Plaid Check products. Default value is 365; maximum is 730; minimum is 180. If a value lower than 180 is provided, a minimum of 180 days of history will be requested.
    #[serde(rename = "days_requested")]
    pub days_requested: i32,
    /// The minimum number of days of data required for the report to be successfully generated.
    #[serde(rename = "days_required", skip_serializing_if = "Option::is_none")]
    pub days_required: Option<i32>,
    #[serde(rename = "partner_insights", skip_serializing_if = "Option::is_none")]
    pub partner_insights: Option<Box<models::LinkTokenCreateRequestCraOptionsPartnerInsights>>,
    #[serde(rename = "base_report", skip_serializing_if = "Option::is_none")]
    pub base_report: Option<Box<models::LinkTokenCreateRequestCraOptionsBaseReport>>,
    #[serde(rename = "cashflow_insights", skip_serializing_if = "Option::is_none")]
    pub cashflow_insights: Option<Box<models::LinkTokenCreateRequestCraOptionsCashflowInsights>>,
}

impl LinkTokenCreateRequestCraOptions {
    /// Specifies options for initializing Link for use with Plaid Check products
    pub fn new(days_requested: i32) -> LinkTokenCreateRequestCraOptions {
        LinkTokenCreateRequestCraOptions {
            days_requested,
            days_required: None,
            partner_insights: None,
            base_report: None,
            cashflow_insights: None,
        }
    }
}

