/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferAuthorizationPaymentRisk : This object includes the scores and risk level. This response is offered as an add-on to /transfer/authorization/create. To request access to these fields please contact your Plaid account manager.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferAuthorizationPaymentRisk {
    /// A score from 1-99 that indicates the transaction return risk: a higher risk score suggests a higher return likelihood.  The score evaluates the transaction return risk because an account is overdrawn or because an ineligible account is used and covers return codes: \"R01\", \"R02\", \"R03\", \"R04\", \"R06\", \"R08\",  \"R09\", \"R13\", \"R16\", \"R17\", \"R20\", \"R23\". These returns have a turnaround time of 2 banking days.
    #[serde(rename = "bank_initiated_return_score", deserialize_with = "Option::deserialize")]
    pub bank_initiated_return_score: Option<i32>,
    /// A score from 1-99 that indicates the transaction return risk: a higher risk score suggests a higher return likelihood.  The score evaluates the transaction return risk of an unauthorized debit and covers return codes: \"R05\", \"R07\", \"R10\", \"R11\", \"R29\". These returns typically have a return time frame of up to 60 calendar days. During this period, the customer of financial institutions can dispute a transaction as unauthorized.
    #[serde(rename = "customer_initiated_return_score", deserialize_with = "Option::deserialize")]
    pub customer_initiated_return_score: Option<i32>,
    #[serde(rename = "risk_level", deserialize_with = "Option::deserialize")]
    pub risk_level: Option<crate::models::TransferAuthorizationRiskLevel>,
    /// If bank information was not available to be used in the Signal model, this array contains warnings describing why bank data is missing. If you want to receive an API error instead of Signal scores in the case of missing bank data, file a support ticket or contact your Plaid account manager.
    #[serde(rename = "warnings")]
    pub warnings: Vec<crate::models::SignalWarning>,
}

impl TransferAuthorizationPaymentRisk {
    /// This object includes the scores and risk level. This response is offered as an add-on to /transfer/authorization/create. To request access to these fields please contact your Plaid account manager.
    pub fn new(bank_initiated_return_score: Option<i32>, customer_initiated_return_score: Option<i32>, risk_level: Option<crate::models::TransferAuthorizationRiskLevel>, warnings: Vec<crate::models::SignalWarning>) -> TransferAuthorizationPaymentRisk {
        TransferAuthorizationPaymentRisk {
            bank_initiated_return_score,
            customer_initiated_return_score,
            risk_level,
            warnings,
        }
    }
}


