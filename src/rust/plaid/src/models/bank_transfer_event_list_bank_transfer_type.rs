/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BankTransferEventListBankTransferType : The type of bank transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into your origination account; a `credit` indicates a transfer of money out of your origination account.

/// The type of bank transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into your origination account; a `credit` indicates a transfer of money out of your origination account.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BankTransferEventListBankTransferType {
    #[serde(rename = "debit")]
    Debit,
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "null")]
    Null,

}

impl ToString for BankTransferEventListBankTransferType {
    fn to_string(&self) -> String {
        match self {
            Self::Debit => String::from("debit"),
            Self::Credit => String::from("credit"),
            Self::Null => String::from("null"),
        }
    }
}

impl Default for BankTransferEventListBankTransferType {
    fn default() -> BankTransferEventListBankTransferType {
        Self::Debit
    }
}




