/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferSweepStatus : The status of the sweep for the transfer.  `unswept`: The transfer hasn't been swept yet. `swept`: The transfer was swept to the sweep account. `swept_settled`: Credits are available to be withdrawn or debits have been deducted from the customer’s business checking account. `return_swept`: The transfer was returned, funds were pulled back or pushed back to the sweep account. `null`: The transfer will never be swept (e.g. if the transfer is cancelled or returned before being swept)

/// The status of the sweep for the transfer.  `unswept`: The transfer hasn't been swept yet. `swept`: The transfer was swept to the sweep account. `swept_settled`: Credits are available to be withdrawn or debits have been deducted from the customer’s business checking account. `return_swept`: The transfer was returned, funds were pulled back or pushed back to the sweep account. `null`: The transfer will never be swept (e.g. if the transfer is cancelled or returned before being swept)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferSweepStatus {
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "unswept")]
    Unswept,
    #[serde(rename = "swept")]
    Swept,
    #[serde(rename = "swept_settled")]
    SweptSettled,
    #[serde(rename = "return_swept")]
    ReturnSwept,

}

impl ToString for TransferSweepStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Null => String::from("null"),
            Self::Unswept => String::from("unswept"),
            Self::Swept => String::from("swept"),
            Self::SweptSettled => String::from("swept_settled"),
            Self::ReturnSwept => String::from("return_swept"),
        }
    }
}

impl Default for TransferSweepStatus {
    fn default() -> TransferSweepStatus {
        Self::Null
    }
}




