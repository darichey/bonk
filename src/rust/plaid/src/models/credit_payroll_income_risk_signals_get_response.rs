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

/// CreditPayrollIncomeRiskSignalsGetResponse : CreditPayrollIncomeRiskSignalsGetRequest defines the response schema for `/credit/payroll_income/risk_signals/get`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditPayrollIncomeRiskSignalsGetResponse {
    /// Array of payroll items.
    #[serde(rename = "items")]
    pub items: Vec<models::PayrollRiskSignalsItem>,
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error: Option<Option<models::PlaidError>>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl CreditPayrollIncomeRiskSignalsGetResponse {
    /// CreditPayrollIncomeRiskSignalsGetRequest defines the response schema for `/credit/payroll_income/risk_signals/get`
    pub fn new(items: Vec<models::PayrollRiskSignalsItem>, request_id: String) -> CreditPayrollIncomeRiskSignalsGetResponse {
        CreditPayrollIncomeRiskSignalsGetResponse {
            items,
            error: None,
            request_id,
        }
    }
}

