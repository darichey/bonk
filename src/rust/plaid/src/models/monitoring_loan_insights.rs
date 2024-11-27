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

/// MonitoringLoanInsights : An object representing the loan exposure subcategory of the report
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitoringLoanInsights {
    #[serde(rename = "loan_payments_counts")]
    pub loan_payments_counts: models::LoanPaymentsCounts,
    /// The number of loan disbursements detected in the last 30 days
    #[serde(rename = "loan_disbursements_count")]
    pub loan_disbursements_count: f64,
    #[serde(rename = "loan_payment_merchants_counts")]
    pub loan_payment_merchants_counts: models::LoanPaymentsMerchantCounts,
}

impl MonitoringLoanInsights {
    /// An object representing the loan exposure subcategory of the report
    pub fn new(loan_payments_counts: models::LoanPaymentsCounts, loan_disbursements_count: f64, loan_payment_merchants_counts: models::LoanPaymentsMerchantCounts) -> MonitoringLoanInsights {
        MonitoringLoanInsights {
            loan_payments_counts,
            loan_disbursements_count,
            loan_payment_merchants_counts,
        }
    }
}

