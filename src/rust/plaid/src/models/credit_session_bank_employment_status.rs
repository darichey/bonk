/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditSessionBankEmploymentStatus : Status of the Bank Employment Link session.  `APPROVED`: User has approved and verified their employment.  `NO_EMPLOYMENTS_FOUND`: We attempted, but were unable to find any employment in the connected account.  `EMPLOYER_NOT_LISTED`: The user explicitly indicated that they did not see their current or previous employer in the list of employer names found.  `STARTED`: The user began the bank income portion of the link flow.  `INTERNAL_ERROR`: The user encountered an internal error.

/// Status of the Bank Employment Link session.  `APPROVED`: User has approved and verified their employment.  `NO_EMPLOYMENTS_FOUND`: We attempted, but were unable to find any employment in the connected account.  `EMPLOYER_NOT_LISTED`: The user explicitly indicated that they did not see their current or previous employer in the list of employer names found.  `STARTED`: The user began the bank income portion of the link flow.  `INTERNAL_ERROR`: The user encountered an internal error.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreditSessionBankEmploymentStatus {
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "NO_EMPLOYERS_FOUND")]
    NoEmployersFound,
    #[serde(rename = "EMPLOYER_NOT_LISTED")]
    EmployerNotListed,

}

impl ToString for CreditSessionBankEmploymentStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Approved => String::from("APPROVED"),
            Self::NoEmployersFound => String::from("NO_EMPLOYERS_FOUND"),
            Self::EmployerNotListed => String::from("EMPLOYER_NOT_LISTED"),
        }
    }
}

impl Default for CreditSessionBankEmploymentStatus {
    fn default() -> CreditSessionBankEmploymentStatus {
        Self::Approved
    }
}




