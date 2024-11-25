/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OmittableTransferType : The type of transfer. Valid values are `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account. This field is omitted for Plaid Ledger Sweep events.

/// The type of transfer. Valid values are `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account. This field is omitted for Plaid Ledger Sweep events.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OmittableTransferType {
    #[serde(rename = "debit")]
    Debit,
    #[serde(rename = "credit")]
    Credit,

}

impl ToString for OmittableTransferType {
    fn to_string(&self) -> String {
        match self {
            Self::Debit => String::from("debit"),
            Self::Credit => String::from("credit"),
        }
    }
}

impl Default for OmittableTransferType {
    fn default() -> OmittableTransferType {
        Self::Debit
    }
}




