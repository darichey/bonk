/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferDocumentPurpose : Specifies the purpose of the uploaded file.  `\"DUE_DILIGENCE\"` - The transfer due diligence document of the originator.

/// Specifies the purpose of the uploaded file.  `\"DUE_DILIGENCE\"` - The transfer due diligence document of the originator.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferDocumentPurpose {
    #[serde(rename = "DUE_DILIGENCE")]
    DueDiligence,

}

impl ToString for TransferDocumentPurpose {
    fn to_string(&self) -> String {
        match self {
            Self::DueDiligence => String::from("DUE_DILIGENCE"),
        }
    }
}

impl Default for TransferDocumentPurpose {
    fn default() -> TransferDocumentPurpose {
        Self::DueDiligence
    }
}




