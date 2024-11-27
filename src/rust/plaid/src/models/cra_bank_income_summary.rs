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

/// CraBankIncomeSummary : Summary for income across all income sources and items (max history of 730 days).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CraBankIncomeSummary {
    /// Total amount of earnings across all the income sources in the end user's Items for the days requested by the client. This can contain multiple amounts, with each amount denominated in one unique currency.
    #[serde(rename = "total_amounts", skip_serializing_if = "Option::is_none")]
    pub total_amounts: Option<Vec<models::CreditAmountWithCurrency>>,
    /// The earliest date within the days requested in which all income sources identified by Plaid appear in a user's account. The date will be returned in an ISO 8601 format (YYYY-MM-DD).
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// The latest date in which all income sources identified by Plaid appear in the user's account. The date will be returned in an ISO 8601 format (YYYY-MM-DD).
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Number of income sources per end user.
    #[serde(rename = "income_sources_count", skip_serializing_if = "Option::is_none")]
    pub income_sources_count: Option<i32>,
    /// Number of income categories per end user.
    #[serde(rename = "income_categories_count", skip_serializing_if = "Option::is_none")]
    pub income_categories_count: Option<i32>,
    /// Number of income transactions per end user.
    #[serde(rename = "income_transactions_count", skip_serializing_if = "Option::is_none")]
    pub income_transactions_count: Option<i32>,
    /// An estimate of the average gross monthly income based on the historical net amount and income category for the income source(s).
    #[serde(rename = "historical_average_monthly_gross_income", skip_serializing_if = "Option::is_none")]
    pub historical_average_monthly_gross_income: Option<Vec<models::CreditAmountWithCurrency>>,
    /// The average monthly income amount estimated based on the historical data for the income source(s).
    #[serde(rename = "historical_average_monthly_income", skip_serializing_if = "Option::is_none")]
    pub historical_average_monthly_income: Option<Vec<models::CreditAmountWithCurrency>>,
    /// The predicted average monthly income amount for the income source(s).
    #[serde(rename = "forecasted_average_monthly_income", skip_serializing_if = "Option::is_none")]
    pub forecasted_average_monthly_income: Option<Vec<models::CreditAmountWithCurrency>>,
    /// An estimate of the annual gross income based on the historical net amount and income category for the income source(s).
    #[serde(rename = "historical_annual_gross_income", skip_serializing_if = "Option::is_none")]
    pub historical_annual_gross_income: Option<Vec<models::CreditAmountWithCurrency>>,
    /// The annual income amount estimated based on the historical data for the income source(s).
    #[serde(rename = "historical_annual_income", skip_serializing_if = "Option::is_none")]
    pub historical_annual_income: Option<Vec<models::CreditAmountWithCurrency>>,
    /// The predicted average annual income amount for the income source(s).
    #[serde(rename = "forecasted_annual_income", skip_serializing_if = "Option::is_none")]
    pub forecasted_annual_income: Option<Vec<models::CreditAmountWithCurrency>>,
    #[serde(rename = "historical_summary", skip_serializing_if = "Option::is_none")]
    pub historical_summary: Option<Vec<models::CraBankIncomeHistoricalSummary>>,
}

impl CraBankIncomeSummary {
    /// Summary for income across all income sources and items (max history of 730 days).
    pub fn new() -> CraBankIncomeSummary {
        CraBankIncomeSummary {
            total_amounts: None,
            start_date: None,
            end_date: None,
            income_sources_count: None,
            income_categories_count: None,
            income_transactions_count: None,
            historical_average_monthly_gross_income: None,
            historical_average_monthly_income: None,
            forecasted_average_monthly_income: None,
            historical_annual_gross_income: None,
            historical_annual_income: None,
            forecasted_annual_income: None,
            historical_summary: None,
        }
    }
}

