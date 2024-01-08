/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaystubDetails : An object representing details that can be found on the paystub.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaystubDetails {
    /// Beginning date of the pay period on the paystub in the 'YYYY-MM-DD' format.
    #[serde(rename = "pay_period_start_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pay_period_start_date: Option<Option<String>>,
    /// Ending date of the pay period on the paystub in the 'YYYY-MM-DD' format.
    #[serde(rename = "pay_period_end_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pay_period_end_date: Option<Option<String>>,
    /// Pay date on the paystub in the 'YYYY-MM-DD' format.
    #[serde(rename = "pay_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pay_date: Option<Option<String>>,
    /// The name of the payroll provider that generated the paystub, e.g. ADP
    #[serde(rename = "paystub_provider", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub paystub_provider: Option<Option<String>>,
    #[serde(rename = "pay_frequency", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pay_frequency: Option<Option<crate::models::PaystubPayFrequency>>,
}

impl PaystubDetails {
    /// An object representing details that can be found on the paystub.
    pub fn new() -> PaystubDetails {
        PaystubDetails {
            pay_period_start_date: None,
            pay_period_end_date: None,
            pay_date: None,
            paystub_provider: None,
            pay_frequency: None,
        }
    }
}


