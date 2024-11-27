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

/// ProductStatus : A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductStatus {
    /// This field is deprecated in favor of the `breakdown` object, which provides more granular institution health data.  `HEALTHY`: the majority of requests are successful `DEGRADED`: only some requests are successful `DOWN`: all requests are failing
    #[serde(rename = "status")]
    pub status: Status,
    /// [ISO 8601](https://wikipedia.org/wiki/ISO_8601) formatted timestamp of the last status change for the institution. 
    #[serde(rename = "last_status_change")]
    pub last_status_change: String,
    #[serde(rename = "breakdown")]
    pub breakdown: models::ProductStatusBreakdown,
}

impl ProductStatus {
    /// A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub fn new(status: Status, last_status_change: String, breakdown: models::ProductStatusBreakdown) -> ProductStatus {
        ProductStatus {
            status,
            last_status_change,
            breakdown,
        }
    }
}
/// This field is deprecated in favor of the `breakdown` object, which provides more granular institution health data.  `HEALTHY`: the majority of requests are successful `DEGRADED`: only some requests are successful `DOWN`: all requests are failing
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "HEALTHY")]
    Healthy,
    #[serde(rename = "DEGRADED")]
    Degraded,
    #[serde(rename = "DOWN")]
    Down,
}

impl Default for Status {
    fn default() -> Status {
        Self::Healthy
    }
}

