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

/// PayPeriodDetails : Details about the pay period.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayPeriodDetails {
    /// The amount of the paycheck.
    #[serde(rename = "check_amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub check_amount: Option<Option<f64>>,
    #[serde(rename = "distribution_breakdown", skip_serializing_if = "Option::is_none")]
    pub distribution_breakdown: Option<Vec<models::DistributionBreakdown>>,
    /// The pay period end date, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format: \"yyyy-mm-dd\".
    #[serde(rename = "end_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Option<String>>,
    /// Total earnings before tax/deductions.
    #[serde(rename = "gross_earnings", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub gross_earnings: Option<Option<f64>>,
    /// The date on which the paystub was issued, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (\"yyyy-mm-dd\").
    #[serde(rename = "pay_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pay_date: Option<Option<String>>,
    #[serde(rename = "pay_frequency", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pay_frequency: Option<Option<models::PayPeriodDetailsPayFrequency>>,
    /// The date on which the paystub was issued, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (\"yyyy-mm-dd\").
    #[serde(rename = "pay_day", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pay_day: Option<Option<String>>,
    /// The pay period start date, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format: \"yyyy-mm-dd\".
    #[serde(rename = "start_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Option<String>>,
}

impl PayPeriodDetails {
    /// Details about the pay period.
    pub fn new() -> PayPeriodDetails {
        PayPeriodDetails {
            check_amount: None,
            distribution_breakdown: None,
            end_date: None,
            gross_earnings: None,
            pay_date: None,
            pay_frequency: None,
            pay_day: None,
            start_date: None,
        }
    }
}

