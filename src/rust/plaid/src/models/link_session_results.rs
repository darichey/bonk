/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkSessionResults : The set of results for a Link session.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkSessionResults {
    /// The set of Item adds for the Link session.
    #[serde(rename = "item_add_results")]
    pub item_add_results: Vec<crate::models::LinkSessionItemAddResult>,
    /// The set of Plaid Check Item adds for the Link session.
    #[serde(rename = "cra_item_add_results")]
    pub cra_item_add_results: Vec<crate::models::LinkSessionCraItemAddResult>,
    /// The set of bank income verifications for the Link session.
    #[serde(rename = "bank_income_results")]
    pub bank_income_results: Vec<crate::models::LinkSessionBankIncomeResult>,
    /// The set of payroll income verifications for the Link session.
    #[serde(rename = "payroll_income_results")]
    pub payroll_income_results: Vec<crate::models::LinkSessionPayrollIncomeResult>,
    #[serde(rename = "document_income_results", deserialize_with = "Option::deserialize")]
    pub document_income_results: Option<Box<crate::models::CreditSessionDocumentIncomeResult>>,
}

impl LinkSessionResults {
    /// The set of results for a Link session.
    pub fn new(item_add_results: Vec<crate::models::LinkSessionItemAddResult>, cra_item_add_results: Vec<crate::models::LinkSessionCraItemAddResult>, bank_income_results: Vec<crate::models::LinkSessionBankIncomeResult>, payroll_income_results: Vec<crate::models::LinkSessionPayrollIncomeResult>, document_income_results: Option<crate::models::CreditSessionDocumentIncomeResult>) -> LinkSessionResults {
        LinkSessionResults {
            item_add_results,
            cra_item_add_results,
            bank_income_results,
            payroll_income_results,
            document_income_results: if let Some(x) = document_income_results {Some(Box::new(x))} else {None},
        }
    }
}


