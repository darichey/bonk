/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankStatementUploadBankAccount : An object containing data about a user's bank account related to an uploaded bank statement.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditBankStatementUploadBankAccount {
    /// The name of the bank account
    #[serde(rename = "name", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
    /// The name of the bank institution.
    #[serde(rename = "bank_name", deserialize_with = "Option::deserialize")]
    pub bank_name: Option<String>,
    /// The type of the bank account.
    #[serde(rename = "account_type", deserialize_with = "Option::deserialize")]
    pub account_type: Option<String>,
    /// The bank account number.
    #[serde(rename = "account_number", deserialize_with = "Option::deserialize")]
    pub account_number: Option<String>,
    #[serde(rename = "owner")]
    pub owner: crate::models::CreditBankStatementUploadAccountOwner,
    /// An array of period objects, containing more data on the overall period of the statement.
    #[serde(rename = "periods")]
    pub periods: Vec<crate::models::CreditBankStatementUploadBankAccountPeriod>,
    /// The unique id of the bank account
    #[serde(rename = "account_id", deserialize_with = "Option::deserialize")]
    pub account_id: Option<String>,
}

impl CreditBankStatementUploadBankAccount {
    /// An object containing data about a user's bank account related to an uploaded bank statement.
    pub fn new(name: Option<String>, bank_name: Option<String>, account_type: Option<String>, account_number: Option<String>, owner: crate::models::CreditBankStatementUploadAccountOwner, periods: Vec<crate::models::CreditBankStatementUploadBankAccountPeriod>, account_id: Option<String>) -> CreditBankStatementUploadBankAccount {
        CreditBankStatementUploadBankAccount {
            name,
            bank_name,
            account_type,
            account_number,
            owner,
            periods,
            account_id,
        }
    }
}


