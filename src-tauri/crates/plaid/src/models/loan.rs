/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Loan : Information specific to a mortgage loan agreement between one or more borrowers and a mortgage lender.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Loan {
    #[serde(rename = "LOAN_IDENTIFIERS")]
    pub loan_identifiers: crate::models::LoanIdentifiers,
}

impl Loan {
    /// Information specific to a mortgage loan agreement between one or more borrowers and a mortgage lender.
    pub fn new(loan_identifiers: crate::models::LoanIdentifiers) -> Loan {
        Loan {
            loan_identifiers,
        }
    }
}


