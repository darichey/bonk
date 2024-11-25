/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BaseReportAccountInsights : Calculated insights derived from transaction-level data.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BaseReportAccountInsights {
    /// Date of the earliest transaction in the base report for the account.
    #[serde(rename = "oldest_transaction_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub oldest_transaction_date: Option<Option<String>>,
    /// Date of the most recent transaction in the base report for the account.
    #[serde(rename = "most_recent_transaction_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub most_recent_transaction_date: Option<Option<String>>,
    /// Number of days days available in the base report for the account.
    #[serde(rename = "days_available", skip_serializing_if = "Option::is_none")]
    pub days_available: Option<i32>,
    /// Average number of days between sequential transactions
    #[serde(rename = "average_days_between_transactions", skip_serializing_if = "Option::is_none")]
    pub average_days_between_transactions: Option<f32>,
    /// Deprecated; use `longest_gaps_between_transactions` instead. Longest gap between sequential transactions
    #[serde(rename = "longest_gap_between_transactions", skip_serializing_if = "Option::is_none")]
    pub longest_gap_between_transactions: Option<Vec<crate::models::BaseReportLongestGapInsights>>,
    /// Customers must transition from `longest_gap_between_transactions` by January 31st 2025. Longest gap between sequential transactions in a time period. This array can include multiple time periods.
    #[serde(rename = "longest_gaps_between_transactions", skip_serializing_if = "Option::is_none")]
    pub longest_gaps_between_transactions: Option<Vec<crate::models::BaseReportLongestGapInsights>>,
    /// The number of debits into the account. This array will be empty for non-depository accounts.
    #[serde(rename = "number_of_inflows", skip_serializing_if = "Option::is_none")]
    pub number_of_inflows: Option<Vec<crate::models::BaseReportNumberFlowInsights>>,
    /// Deprecated; use `average_inflow_amounts` instead. Average amount of debit transactions into the account. This array will be empty for non-depository accounts. This field only takes into account USD transactions from the account.
    #[serde(rename = "average_inflow_amount", skip_serializing_if = "Option::is_none")]
    pub average_inflow_amount: Option<Vec<crate::models::BaseReportAverageFlowInsights>>,
    /// Customers must transition from `average_inflow_amount` by January 31st 2025. Average amount of debit transactions into the account in a time period. This array will be empty for non-depository accounts. This field only takes into account USD transactions from the account.
    #[serde(rename = "average_inflow_amounts", skip_serializing_if = "Option::is_none")]
    pub average_inflow_amounts: Option<Vec<crate::models::BaseReportAverageFlowInsights>>,
    /// The number of outflows from the account. This array will be empty for non-depository accounts.
    #[serde(rename = "number_of_outflows", skip_serializing_if = "Option::is_none")]
    pub number_of_outflows: Option<Vec<crate::models::BaseReportNumberFlowInsights>>,
    /// Deprecated; use `average_outflow_amounts` instead. Average amount of transactions out of the account. This array will be empty for non-depository accounts. This field only takes into account USD transactions from the account.
    #[serde(rename = "average_outflow_amount", skip_serializing_if = "Option::is_none")]
    pub average_outflow_amount: Option<Vec<crate::models::BaseReportAverageFlowInsights>>,
    /// Customers must transition from `average_outflow_amount` by January 31st 2025. Average amount of transactions out of the account in a time period. This array will be empty for non-depository accounts. This field only takes into account USD transactions from the account.
    #[serde(rename = "average_outflow_amounts", skip_serializing_if = "Option::is_none")]
    pub average_outflow_amounts: Option<Vec<crate::models::BaseReportAverageFlowInsights>>,
    /// Number of days with no transactions
    #[serde(rename = "number_of_days_no_transactions", skip_serializing_if = "Option::is_none")]
    pub number_of_days_no_transactions: Option<i32>,
}

impl BaseReportAccountInsights {
    /// Calculated insights derived from transaction-level data.
    pub fn new() -> BaseReportAccountInsights {
        BaseReportAccountInsights {
            oldest_transaction_date: None,
            most_recent_transaction_date: None,
            days_available: None,
            average_days_between_transactions: None,
            longest_gap_between_transactions: None,
            longest_gaps_between_transactions: None,
            number_of_inflows: None,
            average_inflow_amount: None,
            average_inflow_amounts: None,
            number_of_outflows: None,
            average_outflow_amount: None,
            average_outflow_amounts: None,
            number_of_days_no_transactions: None,
        }
    }
}


