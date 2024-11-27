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

/// TransferPlatformRequirement : A piece of information that is outstanding for the scaled platform onboarding process.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferPlatformRequirement {
    /// The type of requirement.
    #[serde(rename = "requirement_type", skip_serializing_if = "Option::is_none")]
    pub requirement_type: Option<String>,
    /// UUID of the person associated with the requirement. Only present for individual-scoped requirements.
    #[serde(rename = "person_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub person_id: Option<Option<String>>,
}

impl TransferPlatformRequirement {
    /// A piece of information that is outstanding for the scaled platform onboarding process.
    pub fn new() -> TransferPlatformRequirement {
        TransferPlatformRequirement {
            requirement_type: None,
            person_id: None,
        }
    }
}

