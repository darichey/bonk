/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DocType : The type of document.  `DOCUMENT_TYPE_PAYSTUB`: A paystub.  `DOCUMENT_TYPE_BANK_STATEMENT`: A bank statement.  `DOCUMENT_TYPE_US_TAX_W2`: A W-2 wage and tax statement provided by a US employer reflecting wages earned by the employee.  `DOCUMENT_TYPE_US_MILITARY_ERAS`: An electronic Retirement Account Statement (eRAS) issued by the US military.  `DOCUMENT_TYPE_US_MILITARY_LES`: A Leave and Earnings Statement (LES) issued by the US military.  `DOCUMENT_TYPE_US_MILITARY_CLES`: A Civilian Leave and Earnings Statement (CLES) issued by the US military.  `DOCUMENT_TYPE_GIG`: Used to indicate that the income is related to gig work. Does not necessarily correspond to a specific document type.  `DOCUMENT_TYPE_NONE`: Used to indicate that there is no underlying document for the data.  `DOCUMENT_TYPE_PLAID_GENERATED_PAYSTUB_PDF`: Used to indicate that the PDF for the paystub was generated by Plaid.  `UNKNOWN`: Document type could not be determined.

/// The type of document.  `DOCUMENT_TYPE_PAYSTUB`: A paystub.  `DOCUMENT_TYPE_BANK_STATEMENT`: A bank statement.  `DOCUMENT_TYPE_US_TAX_W2`: A W-2 wage and tax statement provided by a US employer reflecting wages earned by the employee.  `DOCUMENT_TYPE_US_MILITARY_ERAS`: An electronic Retirement Account Statement (eRAS) issued by the US military.  `DOCUMENT_TYPE_US_MILITARY_LES`: A Leave and Earnings Statement (LES) issued by the US military.  `DOCUMENT_TYPE_US_MILITARY_CLES`: A Civilian Leave and Earnings Statement (CLES) issued by the US military.  `DOCUMENT_TYPE_GIG`: Used to indicate that the income is related to gig work. Does not necessarily correspond to a specific document type.  `DOCUMENT_TYPE_NONE`: Used to indicate that there is no underlying document for the data.  `DOCUMENT_TYPE_PLAID_GENERATED_PAYSTUB_PDF`: Used to indicate that the PDF for the paystub was generated by Plaid.  `UNKNOWN`: Document type could not be determined.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DocType {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "DOCUMENT_TYPE_PAYSTUB")]
    DocumentTypePaystub,
    #[serde(rename = "DOCUMENT_TYPE_BANK_STATEMENT")]
    DocumentTypeBankStatement,
    #[serde(rename = "DOCUMENT_TYPE_US_TAX_W2")]
    DocumentTypeUsTaxW2,
    #[serde(rename = "DOCUMENT_TYPE_US_MILITARY_ERAS")]
    DocumentTypeUsMilitaryEras,
    #[serde(rename = "DOCUMENT_TYPE_US_MILITARY_LES")]
    DocumentTypeUsMilitaryLes,
    #[serde(rename = "DOCUMENT_TYPE_US_MILITARY_CLES")]
    DocumentTypeUsMilitaryCles,
    #[serde(rename = "DOCUMENT_TYPE_GIG")]
    DocumentTypeGig,
    #[serde(rename = "DOCUMENT_TYPE_NONE")]
    DocumentTypeNone,
    #[serde(rename = "DOCUMENT_TYPE_US_TAX_1099_MISC")]
    DocumentTypeUsTax1099Misc,
    #[serde(rename = "DOCUMENT_TYPE_US_TAX_1099_K")]
    DocumentTypeUsTax1099K,
    #[serde(rename = "DOCUMENT_TYPE_PLAID_GENERATED_PAYSTUB_PDF")]
    DocumentTypePlaidGeneratedPaystubPdf,

}

impl ToString for DocType {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("UNKNOWN"),
            Self::DocumentTypePaystub => String::from("DOCUMENT_TYPE_PAYSTUB"),
            Self::DocumentTypeBankStatement => String::from("DOCUMENT_TYPE_BANK_STATEMENT"),
            Self::DocumentTypeUsTaxW2 => String::from("DOCUMENT_TYPE_US_TAX_W2"),
            Self::DocumentTypeUsMilitaryEras => String::from("DOCUMENT_TYPE_US_MILITARY_ERAS"),
            Self::DocumentTypeUsMilitaryLes => String::from("DOCUMENT_TYPE_US_MILITARY_LES"),
            Self::DocumentTypeUsMilitaryCles => String::from("DOCUMENT_TYPE_US_MILITARY_CLES"),
            Self::DocumentTypeGig => String::from("DOCUMENT_TYPE_GIG"),
            Self::DocumentTypeNone => String::from("DOCUMENT_TYPE_NONE"),
            Self::DocumentTypeUsTax1099Misc => String::from("DOCUMENT_TYPE_US_TAX_1099_MISC"),
            Self::DocumentTypeUsTax1099K => String::from("DOCUMENT_TYPE_US_TAX_1099_K"),
            Self::DocumentTypePlaidGeneratedPaystubPdf => String::from("DOCUMENT_TYPE_PLAID_GENERATED_PAYSTUB_PDF"),
        }
    }
}

impl Default for DocType {
    fn default() -> DocType {
        Self::Unknown
    }
}




