/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InflowModel : The `inflow_model` allows you to model a test account that receives regular income or make regular payments on a loan. Any transactions generated by the `inflow_model` will appear in addition to randomly generated test data or transactions specified by `override_accounts`.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InflowModel {
    /// Inflow model. One of the following:  `none`: No income  `monthly-income`: Income occurs once per month `monthly-balance-payment`: Pays off the balance on a liability account at the given statement day of month.  `monthly-interest-only-payment`: Makes an interest-only payment on a liability account at the given statement day of month.  Note that account types supported by Liabilities will accrue interest in the Sandbox. The types impacted are account type `credit` with subtype `credit` or `paypal`, and account type `loan` with subtype `student` or `mortgage`.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Amount of income per month. This value is required if `type` is `monthly-income`.
    #[serde(rename = "income_amount")]
    pub income_amount: f64,
    /// Number between 1 and 28, or `last` meaning the last day of the month. The day of the month on which the income transaction will appear. The name of the income transaction. This field is required if `type` is `monthly-income`, `monthly-balance-payment` or `monthly-interest-only-payment`.
    #[serde(rename = "payment_day_of_month")]
    pub payment_day_of_month: f32,
    /// The name of the income transaction. This field is required if `type` is `monthly-income`, `monthly-balance-payment` or `monthly-interest-only-payment`.
    #[serde(rename = "transaction_name")]
    pub transaction_name: String,
    /// Number between 1 and 28, or `last` meaning the last day of the month. The day of the month on which the balance is calculated for the next payment. The name of the income transaction. This field is required if `type` is `monthly-balance-payment` or `monthly-interest-only-payment`.
    #[serde(rename = "statement_day_of_month")]
    pub statement_day_of_month: String,
}

impl InflowModel {
    /// The `inflow_model` allows you to model a test account that receives regular income or make regular payments on a loan. Any transactions generated by the `inflow_model` will appear in addition to randomly generated test data or transactions specified by `override_accounts`.
    pub fn new(r#type: String, income_amount: f64, payment_day_of_month: f32, transaction_name: String, statement_day_of_month: String) -> InflowModel {
        InflowModel {
            r#type,
            income_amount,
            payment_day_of_month,
            transaction_name,
            statement_day_of_month,
        }
    }
}


