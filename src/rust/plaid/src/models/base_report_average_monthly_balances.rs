/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BaseReportAverageMonthlyBalances : Average balance in dollar amount per month



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BaseReportAverageMonthlyBalances {
    /// The start date of this time period. The date will be returned in an ISO 8601 format (YYYY-MM-DD).
    #[serde(rename = "start_date")]
    pub start_date: String,
    /// The end date of this time period. The date will be returned in an ISO 8601 format (YYYY-MM-DD).
    #[serde(rename = "end_date")]
    pub end_date: String,
    #[serde(rename = "average_balance")]
    pub average_balance: crate::models::CreditAmountWithCurrency,
}

impl BaseReportAverageMonthlyBalances {
    /// Average balance in dollar amount per month
    pub fn new(start_date: String, end_date: String, average_balance: crate::models::CreditAmountWithCurrency) -> BaseReportAverageMonthlyBalances {
        BaseReportAverageMonthlyBalances {
            start_date,
            end_date,
            average_balance,
        }
    }
}


