/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LiabilityOverride : Used to configure Sandbox test data for the Liabilities product



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiabilityOverride {
    /// The type of the liability object, either `credit` or `student`. Mortgages are not currently supported in the custom Sandbox.
    #[serde(rename = "type")]
    pub r#type: String,
    /// The purchase APR percentage value. For simplicity, this is the only interest rate used to calculate interest charges. Can only be set if `type` is `credit`.
    #[serde(rename = "purchase_apr")]
    pub purchase_apr: f64,
    /// The cash APR percentage value. Can only be set if `type` is `credit`.
    #[serde(rename = "cash_apr")]
    pub cash_apr: f64,
    /// The balance transfer APR percentage value. Can only be set if `type` is `credit`.
    #[serde(rename = "balance_transfer_apr")]
    pub balance_transfer_apr: f64,
    /// The special APR percentage value. Can only be set if `type` is `credit`.
    #[serde(rename = "special_apr")]
    pub special_apr: f64,
    /// Override the `last_payment_amount` field. Can only be set if `type` is `credit`.
    #[serde(rename = "last_payment_amount")]
    pub last_payment_amount: f64,
    /// Override the `minimum_payment_amount` field. Can only be set if `type` is `credit` or `student`.
    #[serde(rename = "minimum_payment_amount")]
    pub minimum_payment_amount: f64,
    /// Override the `is_overdue` field
    #[serde(rename = "is_overdue")]
    pub is_overdue: bool,
    /// The date on which the loan was initially lent, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format. Can only be set if `type` is `student`.
    #[serde(rename = "origination_date")]
    pub origination_date: String,
    /// The original loan principal. Can only be set if `type` is `student`.
    #[serde(rename = "principal")]
    pub principal: f64,
    /// The interest rate on the loan as a percentage. Can only be set if `type` is `student`.
    #[serde(rename = "nominal_apr")]
    pub nominal_apr: f64,
    /// If set, interest capitalization begins at the given number of months after loan origination. By default interest is never capitalized. Can only be set if `type` is `student`.
    #[serde(rename = "interest_capitalization_grace_period_months")]
    pub interest_capitalization_grace_period_months: f32,
    #[serde(rename = "repayment_model")]
    pub repayment_model: crate::models::StudentLoanRepaymentModel,
    /// Override the `expected_payoff_date` field. Can only be set if `type` is `student`.
    #[serde(rename = "expected_payoff_date")]
    pub expected_payoff_date: String,
    /// Override the `guarantor` field. Can only be set if `type` is `student`.
    #[serde(rename = "guarantor")]
    pub guarantor: String,
    /// Override the `is_federal` field. Can only be set if `type` is `student`.
    #[serde(rename = "is_federal")]
    pub is_federal: bool,
    /// Override the `loan_name` field. Can only be set if `type` is `student`.
    #[serde(rename = "loan_name")]
    pub loan_name: String,
    #[serde(rename = "loan_status")]
    pub loan_status: crate::models::StudentLoanStatus,
    /// Override the `payment_reference_number` field. Can only be set if `type` is `student`.
    #[serde(rename = "payment_reference_number")]
    pub payment_reference_number: String,
    #[serde(rename = "pslf_status")]
    pub pslf_status: crate::models::PslfStatus,
    /// Override the `repayment_plan.description` field. Can only be set if `type` is `student`.
    #[serde(rename = "repayment_plan_description")]
    pub repayment_plan_description: String,
    /// Override the `repayment_plan.type` field. Can only be set if `type` is `student`. Possible values are: `\"extended graduated\"`, `\"extended standard\"`, `\"graduated\"`, `\"income-contingent repayment\"`, `\"income-based repayment\"`, `\"interest only\"`, `\"other\"`, `\"pay as you earn\"`, `\"revised pay as you earn\"`, `\"standard\"`, or `\"saving on a valuable education\"`.
    #[serde(rename = "repayment_plan_type")]
    pub repayment_plan_type: String,
    /// Override the `sequence_number` field. Can only be set if `type` is `student`.
    #[serde(rename = "sequence_number")]
    pub sequence_number: String,
    #[serde(rename = "servicer_address")]
    pub servicer_address: crate::models::Address,
}

impl LiabilityOverride {
    /// Used to configure Sandbox test data for the Liabilities product
    pub fn new(r#type: String, purchase_apr: f64, cash_apr: f64, balance_transfer_apr: f64, special_apr: f64, last_payment_amount: f64, minimum_payment_amount: f64, is_overdue: bool, origination_date: String, principal: f64, nominal_apr: f64, interest_capitalization_grace_period_months: f32, repayment_model: crate::models::StudentLoanRepaymentModel, expected_payoff_date: String, guarantor: String, is_federal: bool, loan_name: String, loan_status: crate::models::StudentLoanStatus, payment_reference_number: String, pslf_status: crate::models::PslfStatus, repayment_plan_description: String, repayment_plan_type: String, sequence_number: String, servicer_address: crate::models::Address) -> LiabilityOverride {
        LiabilityOverride {
            r#type,
            purchase_apr,
            cash_apr,
            balance_transfer_apr,
            special_apr,
            last_payment_amount,
            minimum_payment_amount,
            is_overdue,
            origination_date,
            principal,
            nominal_apr,
            interest_capitalization_grace_period_months,
            repayment_model,
            expected_payoff_date,
            guarantor,
            is_federal,
            loan_name,
            loan_status,
            payment_reference_number,
            pslf_status,
            repayment_plan_description,
            repayment_plan_type,
            sequence_number,
            servicer_address,
        }
    }
}


