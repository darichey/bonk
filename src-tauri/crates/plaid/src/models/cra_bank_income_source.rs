/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraBankIncomeSource : Detailed information for the income source.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CraBankIncomeSource {
    /// A unique identifier for an income source.
    #[serde(rename = "income_source_id", skip_serializing_if = "Option::is_none")]
    pub income_source_id: Option<String>,
    /// The most common name or original description for the underlying income transactions.
    #[serde(rename = "income_description", skip_serializing_if = "Option::is_none")]
    pub income_description: Option<String>,
    #[serde(rename = "income_category", skip_serializing_if = "Option::is_none")]
    pub income_category: Option<crate::models::CreditBankIncomeCategory>,
    /// Minimum of all dates within the specific income sources in the user's bank account for days requested by the client. The date will be returned in an ISO 8601 format (YYYY-MM-DD).
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Maximum of all dates within the specific income sources in the user’s bank account for days requested by the client. The date will be returned in an ISO 8601 format (YYYY-MM-DD).
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "pay_frequency", skip_serializing_if = "Option::is_none")]
    pub pay_frequency: Option<crate::models::CreditBankIncomePayFrequency>,
    /// Total amount of earnings in the user’s bank account for the specific income source for days requested by the client.
    #[serde(rename = "total_amount", skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f32>,
    /// The ISO 4217 currency code of the amount or balance.
    #[serde(rename = "iso_currency_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<Option<String>>,
    /// The unofficial currency code associated with the amount or balance. Always `null` if `iso_currency_code` is non-null. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.
    #[serde(rename = "unofficial_currency_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<Option<String>>,
    /// Number of transactions for the income source within the start and end date.
    #[serde(rename = "transaction_count", skip_serializing_if = "Option::is_none")]
    pub transaction_count: Option<i32>,
    /// The expected date of the end user’s next paycheck for the income source. The date will be returned in an ISO 8601 format (YYYY-MM-DD).
    #[serde(rename = "next_payment_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub next_payment_date: Option<Option<String>>,
    /// An estimate of the average gross monthly income based on the historical net amount and income category for the income source(s).
    #[serde(rename = "historical_average_monthly_gross_income", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub historical_average_monthly_gross_income: Option<Option<f32>>,
    /// The average monthly net income amount estimated based on the historical data for the income source(s).
    #[serde(rename = "historical_average_monthly_income", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub historical_average_monthly_income: Option<Option<f32>>,
    /// The predicted average monthly net income amount for the income source(s).
    #[serde(rename = "forecasted_average_monthly_income", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub forecasted_average_monthly_income: Option<Option<f32>>,
    #[serde(rename = "employer", skip_serializing_if = "Option::is_none")]
    pub employer: Option<crate::models::CraBankIncomeEmployer>,
    #[serde(rename = "historical_summary", skip_serializing_if = "Option::is_none")]
    pub historical_summary: Option<Vec<crate::models::CraBankIncomeHistoricalSummary>>,
}

impl CraBankIncomeSource {
    /// Detailed information for the income source.
    pub fn new() -> CraBankIncomeSource {
        CraBankIncomeSource {
            income_source_id: None,
            income_description: None,
            income_category: None,
            start_date: None,
            end_date: None,
            pay_frequency: None,
            total_amount: None,
            iso_currency_code: None,
            unofficial_currency_code: None,
            transaction_count: None,
            next_payment_date: None,
            historical_average_monthly_gross_income: None,
            historical_average_monthly_income: None,
            forecasted_average_monthly_income: None,
            employer: None,
            historical_summary: None,
        }
    }
}


