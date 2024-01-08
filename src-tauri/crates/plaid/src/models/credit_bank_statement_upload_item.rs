/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankStatementUploadItem : An object containing information about the bank statement upload Item.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditBankStatementUploadItem {
    /// The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[serde(rename = "bank_statements")]
    pub bank_statements: Vec<crate::models::CreditBankStatementUploadObject>,
    #[serde(rename = "status", deserialize_with = "Option::deserialize")]
    pub status: Option<crate::models::PayrollItemStatus>,
    /// Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DDTHH:mm:ssZ) indicating the last time that the Item was updated.
    #[serde(rename = "updated_at", deserialize_with = "Option::deserialize")]
    pub updated_at: Option<String>,
}

impl CreditBankStatementUploadItem {
    /// An object containing information about the bank statement upload Item.
    pub fn new(item_id: String, bank_statements: Vec<crate::models::CreditBankStatementUploadObject>, status: Option<crate::models::PayrollItemStatus>, updated_at: Option<String>) -> CreditBankStatementUploadItem {
        CreditBankStatementUploadItem {
            item_id,
            bank_statements,
            status,
            updated_at,
        }
    }
}


