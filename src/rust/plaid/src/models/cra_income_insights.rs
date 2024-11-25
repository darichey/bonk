/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraIncomeInsights : The Check Income Insights Report for an end user.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CraIncomeInsights {
    /// The unique identifier associated with the Check Income Insights Report.
    #[serde(rename = "report_id", skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    /// The time when the Check Income Insights Report was generated.
    #[serde(rename = "generated_time", skip_serializing_if = "Option::is_none")]
    pub generated_time: Option<String>,
    /// The number of days requested by the customer for the Check Income Insights Report.
    #[serde(rename = "days_requested", skip_serializing_if = "Option::is_none")]
    pub days_requested: Option<i32>,
    /// The list of Items in the report along with the associated metadata about the Item.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::CraBankIncomeItem>>,
    #[serde(rename = "bank_income_summary", skip_serializing_if = "Option::is_none")]
    pub bank_income_summary: Option<crate::models::CraBankIncomeSummary>,
    /// If data from the report was unable to be retrieved, the warnings object will contain information about the error that caused the data to be incomplete.
    #[serde(rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<crate::models::CraBankIncomeWarning>>,
}

impl CraIncomeInsights {
    /// The Check Income Insights Report for an end user.
    pub fn new() -> CraIncomeInsights {
        CraIncomeInsights {
            report_id: None,
            generated_time: None,
            days_requested: None,
            items: None,
            bank_income_summary: None,
            warnings: None,
        }
    }
}


