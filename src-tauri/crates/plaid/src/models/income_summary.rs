/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IncomeSummary : The verified fields from a paystub verification. All fields are provided as reported on the paystub.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncomeSummary {
    #[serde(rename = "employer_name")]
    pub employer_name: Box<crate::models::EmployerIncomeSummaryFieldString>,
    #[serde(rename = "employee_name")]
    pub employee_name: Box<crate::models::EmployeeIncomeSummaryFieldString>,
    #[serde(rename = "ytd_gross_income")]
    pub ytd_gross_income: Box<crate::models::YtdGrossIncomeSummaryFieldNumber>,
    #[serde(rename = "ytd_net_income")]
    pub ytd_net_income: Box<crate::models::YtdNetIncomeSummaryFieldNumber>,
    #[serde(rename = "pay_frequency", deserialize_with = "Option::deserialize")]
    pub pay_frequency: Option<crate::models::PayFrequency>,
    #[serde(rename = "projected_wage")]
    pub projected_wage: Box<crate::models::ProjectedIncomeSummaryFieldNumber>,
    #[serde(rename = "verified_transaction", deserialize_with = "Option::deserialize")]
    pub verified_transaction: Option<crate::models::TransactionData>,
}

impl IncomeSummary {
    /// The verified fields from a paystub verification. All fields are provided as reported on the paystub.
    pub fn new(employer_name: crate::models::EmployerIncomeSummaryFieldString, employee_name: crate::models::EmployeeIncomeSummaryFieldString, ytd_gross_income: crate::models::YtdGrossIncomeSummaryFieldNumber, ytd_net_income: crate::models::YtdNetIncomeSummaryFieldNumber, pay_frequency: Option<crate::models::PayFrequency>, projected_wage: crate::models::ProjectedIncomeSummaryFieldNumber, verified_transaction: Option<crate::models::TransactionData>) -> IncomeSummary {
        IncomeSummary {
            employer_name: Box::new(employer_name),
            employee_name: Box::new(employee_name),
            ytd_gross_income: Box::new(ytd_gross_income),
            ytd_net_income: Box::new(ytd_net_income),
            pay_frequency,
            projected_wage: Box::new(projected_wage),
            verified_transaction,
        }
    }
}


