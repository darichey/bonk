/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraBankIncomeWarning : The warning associated with the data that was unavailable for the Bank Income Report.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CraBankIncomeWarning {
    #[serde(rename = "warning_type", skip_serializing_if = "Option::is_none")]
    pub warning_type: Option<crate::models::CreditBankIncomeWarningType>,
    #[serde(rename = "warning_code", skip_serializing_if = "Option::is_none")]
    pub warning_code: Option<crate::models::CraBankIncomeWarningCode>,
    #[serde(rename = "cause", skip_serializing_if = "Option::is_none")]
    pub cause: Option<Box<crate::models::CraBankIncomeCause>>,
}

impl CraBankIncomeWarning {
    /// The warning associated with the data that was unavailable for the Bank Income Report.
    pub fn new() -> CraBankIncomeWarning {
        CraBankIncomeWarning {
            warning_type: None,
            warning_code: None,
            cause: None,
        }
    }
}


