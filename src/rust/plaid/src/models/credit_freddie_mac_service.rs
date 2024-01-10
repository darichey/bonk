/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditFreddieMacService : A collection of details related to a fulfillment service or product in terms of request, process and result.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditFreddieMacService {
    #[serde(rename = "VERIFICATION_OF_ASSET")]
    pub verification_of_asset: Vec<crate::models::CreditFreddieMacVerificationOfAsset>,
    #[serde(rename = "STATUSES")]
    pub statuses: crate::models::Statuses,
}

impl CreditFreddieMacService {
    /// A collection of details related to a fulfillment service or product in terms of request, process and result.
    pub fn new(verification_of_asset: Vec<crate::models::CreditFreddieMacVerificationOfAsset>, statuses: crate::models::Statuses) -> CreditFreddieMacService {
        CreditFreddieMacService {
            verification_of_asset,
            statuses,
        }
    }
}

