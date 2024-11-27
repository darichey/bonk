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

/// CreditFreddieMacVerificationOfAssetResponse : Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditFreddieMacVerificationOfAssetResponse {
    #[serde(rename = "ASSETS")]
    pub assets: models::CreditFreddieMacAssets,
}

impl CreditFreddieMacVerificationOfAssetResponse {
    /// Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    pub fn new(assets: models::CreditFreddieMacAssets) -> CreditFreddieMacVerificationOfAssetResponse {
        CreditFreddieMacVerificationOfAssetResponse {
            assets,
        }
    }
}

