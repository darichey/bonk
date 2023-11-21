/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditDocumentMetadata : Object representing metadata pertaining to the document.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditDocumentMetadata {
    /// The name of the document.
    #[serde(rename = "name")]
    pub name: String,
    /// The type of document.  `PAYSTUB`: A paystub.  `BANK_STATEMENT`: A bank statement.  `US_TAX_W2`: A W-2 wage and tax statement provided by a US employer reflecting wages earned by the employee.  `US_MILITARY_ERAS`: An electronic Retirement Account Statement (eRAS) issued by the US military.  `US_MILITARY_LES`: A Leave and Earnings Statement (LES) issued by the US military.  `US_MILITARY_CLES`: A Civilian Leave and Earnings Statment (CLES) issued by the US military.  `GIG`: Used to indicate that the income is related to gig work. Does not necessarily correspond to a specific document type.  `PLAID_GENERATED_PAYSTUB_PDF`: Used to indicate that the PDF for the paystub was generated by Plaid.  `NONE`: Used to indicate that there is no underlying document for the data.  `UNKNOWN`: Document type could not be determined.
    #[serde(rename = "document_type", deserialize_with = "Option::deserialize")]
    pub document_type: Option<String>,
    /// Signed URL to retrieve the underlying file. This download URL can only be used once and expires after two minutes. To generate a new download URL, call `/credit/payroll_income/get` again.
    #[serde(rename = "download_url", deserialize_with = "Option::deserialize")]
    pub download_url: Option<String>,
    /// The processing status of the document.  `PROCESSING_COMPLETE`: The document was successfully processed.  `DOCUMENT_ERROR`: The document could not be processed. Possible causes include: The document was an unacceptable document type such as an offer letter or bank statement, the document image was cropped or blurry, or the document was corrupted.  `UNKNOWN` or `null`: An internal error occurred. If this happens repeatedly, contact support or your Plaid account manager.
    #[serde(rename = "status", deserialize_with = "Option::deserialize")]
    pub status: Option<String>,
    /// The number of pages of the uploaded document (if available).
    #[serde(rename = "page_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub page_count: Option<Option<i32>>,
}

impl CreditDocumentMetadata {
    /// Object representing metadata pertaining to the document.
    pub fn new(name: String, document_type: Option<String>, download_url: Option<String>, status: Option<String>) -> CreditDocumentMetadata {
        CreditDocumentMetadata {
            name,
            document_type,
            download_url,
            status,
            page_count: None,
        }
    }
}

