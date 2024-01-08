/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditFreddieMacLoan : Information specific to a mortgage loan agreement between one or more borrowers and a mortgage lender.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditFreddieMacLoan {
    #[serde(rename = "LOAN_IDENTIFIERS")]
    pub loan_identifiers: crate::models::CreditFreddieMacLoanIdentifiers,
    /// Type of loan. The value can only be \"SubjectLoan\"
    #[serde(rename = "LoanRoleType")]
    pub loan_role_type: String,
}

impl CreditFreddieMacLoan {
    /// Information specific to a mortgage loan agreement between one or more borrowers and a mortgage lender.
    pub fn new(loan_identifiers: crate::models::CreditFreddieMacLoanIdentifiers, loan_role_type: String) -> CreditFreddieMacLoan {
        CreditFreddieMacLoan {
            loan_identifiers,
            loan_role_type,
        }
    }
}


