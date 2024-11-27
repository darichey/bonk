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

/// EmploymentVerification : An object containing proof of employment data for an individual
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmploymentVerification {
    #[serde(rename = "status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<models::EmploymentVerificationStatus>>,
    /// Start of employment in ISO 8601 format (YYYY-MM-DD).
    #[serde(rename = "start_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Option<String>>,
    /// End of employment, if applicable. Provided in ISO 8601 format (YYY-MM-DD).
    #[serde(rename = "end_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Option<String>>,
    #[serde(rename = "employer", skip_serializing_if = "Option::is_none")]
    pub employer: Option<models::EmployerVerification>,
    /// Current title of employee.
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "platform_ids", skip_serializing_if = "Option::is_none")]
    pub platform_ids: Option<models::PlatformIds>,
}

impl EmploymentVerification {
    /// An object containing proof of employment data for an individual
    pub fn new() -> EmploymentVerification {
        EmploymentVerification {
            status: None,
            start_date: None,
            end_date: None,
            employer: None,
            title: None,
            platform_ids: None,
        }
    }
}

