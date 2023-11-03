/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankStatementUploadTransaction : An object containing data about a transaction appearing on a user-uploaded bank statement.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditBankStatementUploadTransaction {
    /// The value of the transaction. A negative amount indicates that money moved into the account (such as a paycheck being deposited).
    #[serde(rename = "amount", deserialize_with = "Option::deserialize")]
    pub amount: Option<f32>,
    /// The date of when the transaction was made, in ISO 8601 format (YYYY-MM-DD).
    #[serde(rename = "date", deserialize_with = "Option::deserialize")]
    pub date: Option<String>,
    /// The raw description of the transaction as it appears on the bank statement.
    #[serde(rename = "original_description", deserialize_with = "Option::deserialize")]
    pub original_description: Option<String>,
    /// The unique id of the bank account that this transaction occurs in
    #[serde(rename = "account_id", deserialize_with = "Option::deserialize")]
    pub account_id: Option<String>,
}

impl CreditBankStatementUploadTransaction {
    /// An object containing data about a transaction appearing on a user-uploaded bank statement.
    pub fn new(amount: Option<f32>, date: Option<String>, original_description: Option<String>, account_id: Option<String>) -> CreditBankStatementUploadTransaction {
        CreditBankStatementUploadTransaction {
            amount,
            date,
            original_description,
            account_id,
        }
    }
}


