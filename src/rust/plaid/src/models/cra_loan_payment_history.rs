/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraLoanPaymentHistory : Contains the payment information for a loan payment period.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CraLoanPaymentHistory {
    /// The index to identify the loan's payment period, starting from 1. For example:   1 means the period between the loan's opening date and the 1st payment due date.   2 means the period between the loan's 1st payment due date and 2nd payment due date.
    #[serde(rename = "period")]
    pub period: i32,
    /// The payment due date or end date of the payment period. The date should be in ISO 8601 format (YYYY-MM-DD).
    #[serde(rename = "due_date")]
    pub due_date: String,
    /// The number of days the loan was delinquent at the end of the pay period. If specified, should be greater of equal to 0.
    #[serde(rename = "days_past_due")]
    pub days_past_due: i32,
    /// The amount past due or the charge-off amount of the loan at the end of the payment period.
    #[serde(rename = "amount_past_due", skip_serializing_if = "Option::is_none")]
    pub amount_past_due: Option<f32>,
    /// The balance remaining on the loan at the end of the payment period.
    #[serde(rename = "balance_remaining", skip_serializing_if = "Option::is_none")]
    pub balance_remaining: Option<f32>,
}

impl CraLoanPaymentHistory {
    /// Contains the payment information for a loan payment period.
    pub fn new(period: i32, due_date: String, days_past_due: i32) -> CraLoanPaymentHistory {
        CraLoanPaymentHistory {
            period,
            due_date,
            days_past_due,
            amount_past_due: None,
            balance_remaining: None,
        }
    }
}


