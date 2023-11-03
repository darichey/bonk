/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkSessionSuccessMetadataTransferStatus : The status of a transfer. Returned only when [Transfer UI](/docs/transfer/using-transfer-ui) is implemented.  - `COMPLETE` – The transfer was completed. - `INCOMPLETE` – The transfer could not be completed. For help, see [Troubleshooting transfers](/docs/transfer/using-transfer-ui#troubleshooting-transfers).

/// The status of a transfer. Returned only when [Transfer UI](/docs/transfer/using-transfer-ui) is implemented.  - `COMPLETE` – The transfer was completed. - `INCOMPLETE` – The transfer could not be completed. For help, see [Troubleshooting transfers](/docs/transfer/using-transfer-ui#troubleshooting-transfers).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LinkSessionSuccessMetadataTransferStatus {
    #[serde(rename = "COMPLETE")]
    Complete,
    #[serde(rename = "INCOMPLETE")]
    Incomplete,

}

impl ToString for LinkSessionSuccessMetadataTransferStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Complete => String::from("COMPLETE"),
            Self::Incomplete => String::from("INCOMPLETE"),
        }
    }
}

impl Default for LinkSessionSuccessMetadataTransferStatus {
    fn default() -> LinkSessionSuccessMetadataTransferStatus {
        Self::Complete
    }
}




