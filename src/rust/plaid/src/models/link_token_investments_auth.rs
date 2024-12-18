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

/// LinkTokenInvestmentsAuth : Configuration parameters for the Investments Move product
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokenInvestmentsAuth {
    /// If `true`, show institutions that use the manual entry fallback flow.
    #[serde(rename = "manual_entry_enabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub manual_entry_enabled: Option<Option<bool>>,
    /// If `true`, show institutions that use the masked number match fallback flow.
    #[serde(rename = "masked_number_match_enabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub masked_number_match_enabled: Option<Option<bool>>,
    /// If `true`, show institutions that use the stated account number fallback flow.
    #[serde(rename = "stated_account_number_enabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stated_account_number_enabled: Option<Option<bool>>,
}

impl LinkTokenInvestmentsAuth {
    /// Configuration parameters for the Investments Move product
    pub fn new() -> LinkTokenInvestmentsAuth {
        LinkTokenInvestmentsAuth {
            manual_entry_enabled: None,
            masked_number_match_enabled: None,
            stated_account_number_enabled: None,
        }
    }
}

