/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankStatementUploadObject : An object containing data that has been parsed from a user-uploaded bank statement.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditBankStatementUploadObject {
    /// An array of transactions appearing on the bank statement.
    #[serde(rename = "transactions")]
    pub transactions: Vec<crate::models::CreditBankStatementUploadTransaction>,
    #[serde(rename = "document_metadata")]
    pub document_metadata: crate::models::CreditDocumentMetadata,
    /// An identifier of the document referenced by the document metadata.
    #[serde(rename = "document_id", deserialize_with = "Option::deserialize")]
    pub document_id: Option<String>,
    /// An array of bank accounts associated with the uploaded bank statement.
    #[serde(rename = "bank_accounts")]
    pub bank_accounts: Vec<crate::models::CreditBankStatementUploadBankAccount>,
}

impl CreditBankStatementUploadObject {
    /// An object containing data that has been parsed from a user-uploaded bank statement.
    pub fn new(transactions: Vec<crate::models::CreditBankStatementUploadTransaction>, document_metadata: crate::models::CreditDocumentMetadata, document_id: Option<String>, bank_accounts: Vec<crate::models::CreditBankStatementUploadBankAccount>) -> CreditBankStatementUploadObject {
        CreditBankStatementUploadObject {
            transactions,
            document_metadata,
            document_id,
            bank_accounts,
        }
    }
}

