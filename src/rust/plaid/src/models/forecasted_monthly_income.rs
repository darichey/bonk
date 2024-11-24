/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ForecastedMonthlyIncome : An object representing the predicted average monthly net income amount. This amount reflects the funds deposited into the account and may not include any withheld income such as taxes or other payroll deductions



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ForecastedMonthlyIncome {
    /// The forecasted monthly income at the time of subscription
    #[serde(rename = "baseline_amount")]
    pub baseline_amount: f32,
    /// The current forecasted monthly income
    #[serde(rename = "current_amount")]
    pub current_amount: f32,
}

impl ForecastedMonthlyIncome {
    /// An object representing the predicted average monthly net income amount. This amount reflects the funds deposited into the account and may not include any withheld income such as taxes or other payroll deductions
    pub fn new(baseline_amount: f32, current_amount: f32) -> ForecastedMonthlyIncome {
        ForecastedMonthlyIncome {
            baseline_amount,
            current_amount,
        }
    }
}


