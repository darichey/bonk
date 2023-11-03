/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InstitutionsSearchAccountFilter : An account filter to apply to institutions search requests



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstitutionsSearchAccountFilter {
    #[serde(rename = "loan", skip_serializing_if = "Option::is_none")]
    pub loan: Option<Vec<crate::models::AccountSubtype>>,
    #[serde(rename = "depository", skip_serializing_if = "Option::is_none")]
    pub depository: Option<Vec<crate::models::AccountSubtype>>,
    #[serde(rename = "credit", skip_serializing_if = "Option::is_none")]
    pub credit: Option<Vec<crate::models::AccountSubtype>>,
    #[serde(rename = "investment", skip_serializing_if = "Option::is_none")]
    pub investment: Option<Vec<crate::models::AccountSubtype>>,
}

impl InstitutionsSearchAccountFilter {
    /// An account filter to apply to institutions search requests
    pub fn new() -> InstitutionsSearchAccountFilter {
        InstitutionsSearchAccountFilter {
            loan: None,
            depository: None,
            credit: None,
            investment: None,
        }
    }
}


