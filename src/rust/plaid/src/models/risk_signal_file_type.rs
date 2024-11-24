/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RiskSignalFileType : The file type for risk signal analysis

/// The file type for risk signal analysis
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RiskSignalFileType {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "IMAGE_PDF")]
    ImagePdf,
    #[serde(rename = "SCAN_OCR")]
    ScanOcr,
    #[serde(rename = "TRUE_PDF")]
    TruePdf,
    #[serde(rename = "IMAGE")]
    Image,
    #[serde(rename = "MIXED_PAGE_PDF")]
    MixedPagePdf,
    #[serde(rename = "EMPTY_PDF")]
    EmptyPdf,
    #[serde(rename = "FLATTENED_PDF")]
    FlattenedPdf,

}

impl ToString for RiskSignalFileType {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("UNKNOWN"),
            Self::ImagePdf => String::from("IMAGE_PDF"),
            Self::ScanOcr => String::from("SCAN_OCR"),
            Self::TruePdf => String::from("TRUE_PDF"),
            Self::Image => String::from("IMAGE"),
            Self::MixedPagePdf => String::from("MIXED_PAGE_PDF"),
            Self::EmptyPdf => String::from("EMPTY_PDF"),
            Self::FlattenedPdf => String::from("FLATTENED_PDF"),
        }
    }
}

impl Default for RiskSignalFileType {
    fn default() -> RiskSignalFileType {
        Self::Unknown
    }
}




