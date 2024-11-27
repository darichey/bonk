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

/// KycCheckIdNumberSummary : Result summary object specifying how the `id_number` field matched.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct KycCheckIdNumberSummary {
    #[serde(rename = "summary")]
    pub summary: models::MatchSummaryCode,
}

impl KycCheckIdNumberSummary {
    /// Result summary object specifying how the `id_number` field matched.
    pub fn new(summary: models::MatchSummaryCode) -> KycCheckIdNumberSummary {
        KycCheckIdNumberSummary {
            summary,
        }
    }
}

