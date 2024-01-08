/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankEmploymentWarning : The warning associated with the data that was unavailable for the Bank Employment Report.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditBankEmploymentWarning {
    #[serde(rename = "warning_type")]
    pub warning_type: crate::models::CreditBankEmploymentWarningType,
    #[serde(rename = "warning_code")]
    pub warning_code: crate::models::CreditBankIncomeWarningCode,
    #[serde(rename = "cause")]
    pub cause: Box<crate::models::CreditBankIncomeCause>,
}

impl CreditBankEmploymentWarning {
    /// The warning associated with the data that was unavailable for the Bank Employment Report.
    pub fn new(warning_type: crate::models::CreditBankEmploymentWarningType, warning_code: crate::models::CreditBankIncomeWarningCode, cause: crate::models::CreditBankIncomeCause) -> CreditBankEmploymentWarning {
        CreditBankEmploymentWarning {
            warning_type,
            warning_code,
            cause: Box::new(cause),
        }
    }
}


