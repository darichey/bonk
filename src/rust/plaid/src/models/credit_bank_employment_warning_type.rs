/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankEmploymentWarningType : The warning type which will always be `BANK_EMPLOYMENT_WARNING`.

/// The warning type which will always be `BANK_EMPLOYMENT_WARNING`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreditBankEmploymentWarningType {
    #[serde(rename = "BANK_EMPLOYMENT_WARNING")]
    BankEmploymentWarning,

}

impl ToString for CreditBankEmploymentWarningType {
    fn to_string(&self) -> String {
        match self {
            Self::BankEmploymentWarning => String::from("BANK_EMPLOYMENT_WARNING"),
        }
    }
}

impl Default for CreditBankEmploymentWarningType {
    fn default() -> CreditBankEmploymentWarningType {
        Self::BankEmploymentWarning
    }
}




