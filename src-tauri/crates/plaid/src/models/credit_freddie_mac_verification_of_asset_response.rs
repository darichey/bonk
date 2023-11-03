/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditFreddieMacVerificationOfAssetResponse : Documentation not found in the MISMO model viewer and not provided by Freddie Mac.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditFreddieMacVerificationOfAssetResponse {
    #[serde(rename = "ASSETS")]
    pub assets: crate::models::CreditFreddieMacAssets,
}

impl CreditFreddieMacVerificationOfAssetResponse {
    /// Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    pub fn new(assets: crate::models::CreditFreddieMacAssets) -> CreditFreddieMacVerificationOfAssetResponse {
        CreditFreddieMacVerificationOfAssetResponse {
            assets,
        }
    }
}


