/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenCreateRequestUserStatedIncomeSource : Specifies user stated income sources for the Income product



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestUserStatedIncomeSource {
    /// The employer corresponding to an income source specified by the user
    #[serde(rename = "employer", skip_serializing_if = "Option::is_none")]
    pub employer: Option<String>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<crate::models::UserStatedIncomeSourceCategory>,
    /// The income amount paid per cycle for a specified income source
    #[serde(rename = "pay_per_cycle", skip_serializing_if = "Option::is_none")]
    pub pay_per_cycle: Option<f64>,
    /// The income amount paid annually for a specified income source
    #[serde(rename = "pay_annual", skip_serializing_if = "Option::is_none")]
    pub pay_annual: Option<f64>,
    #[serde(rename = "pay_type", skip_serializing_if = "Option::is_none")]
    pub pay_type: Option<crate::models::UserStatedIncomeSourcePayType>,
    #[serde(rename = "pay_frequency", skip_serializing_if = "Option::is_none")]
    pub pay_frequency: Option<crate::models::UserStatedIncomeSourceFrequency>,
}

impl LinkTokenCreateRequestUserStatedIncomeSource {
    /// Specifies user stated income sources for the Income product
    pub fn new() -> LinkTokenCreateRequestUserStatedIncomeSource {
        LinkTokenCreateRequestUserStatedIncomeSource {
            employer: None,
            category: None,
            pay_per_cycle: None,
            pay_annual: None,
            pay_type: None,
            pay_frequency: None,
        }
    }
}


