/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IncomeVerificationPayrollFlowType : Flow types to retrieve payroll income data

/// Flow types to retrieve payroll income data
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IncomeVerificationPayrollFlowType {
    #[serde(rename = "payroll_digital_income")]
    DigitalIncome,
    #[serde(rename = "payroll_document_income")]
    DocumentIncome,

}

impl ToString for IncomeVerificationPayrollFlowType {
    fn to_string(&self) -> String {
        match self {
            Self::DigitalIncome => String::from("payroll_digital_income"),
            Self::DocumentIncome => String::from("payroll_document_income"),
        }
    }
}

impl Default for IncomeVerificationPayrollFlowType {
    fn default() -> IncomeVerificationPayrollFlowType {
        Self::DigitalIncome
    }
}




