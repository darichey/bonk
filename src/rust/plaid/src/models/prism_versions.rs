/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PrismVersions : The versions of Prism products to evaluate



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PrismVersions {
    #[serde(rename = "firstdetect", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub firstdetect: Option<Option<crate::models::PrismFirstDetectVersion>>,
    #[serde(rename = "cashscore", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cashscore: Option<Option<crate::models::PrismCashScoreVersion>>,
    #[serde(rename = "insights", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub insights: Option<Option<crate::models::PrismInsightsVersion>>,
}

impl PrismVersions {
    /// The versions of Prism products to evaluate
    pub fn new() -> PrismVersions {
        PrismVersions {
            firstdetect: None,
            cashscore: None,
            insights: None,
        }
    }
}


