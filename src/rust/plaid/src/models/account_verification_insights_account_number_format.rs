/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AccountVerificationInsightsAccountNumberFormat : Indicator of account number format validity for institution.  `valid`: indicates that the account number has a correct format for the institution.  `invalid`: indicates that the account number has an incorrect format for the institution.  `unknown`: indicates that there was not enough information to determine whether the format is correct for the institution.

/// Indicator of account number format validity for institution.  `valid`: indicates that the account number has a correct format for the institution.  `invalid`: indicates that the account number has an incorrect format for the institution.  `unknown`: indicates that there was not enough information to determine whether the format is correct for the institution.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountVerificationInsightsAccountNumberFormat {
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "unknown")]
    Unknown,

}

impl ToString for AccountVerificationInsightsAccountNumberFormat {
    fn to_string(&self) -> String {
        match self {
            Self::Valid => String::from("valid"),
            Self::Invalid => String::from("invalid"),
            Self::Unknown => String::from("unknown"),
        }
    }
}

impl Default for AccountVerificationInsightsAccountNumberFormat {
    fn default() -> AccountVerificationInsightsAccountNumberFormat {
        Self::Valid
    }
}



