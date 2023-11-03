/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateIndividualScreeningRequestSearchTerms : Search terms for editing an individual watchlist screening



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateIndividualScreeningRequestSearchTerms {
    /// ID of the associated program.
    #[serde(rename = "watchlist_program_id", skip_serializing_if = "Option::is_none")]
    pub watchlist_program_id: Option<String>,
    /// The legal name of the individual being screened.
    #[serde(rename = "legal_name", skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    /// A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(rename = "date_of_birth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// The numeric or alphanumeric identifier associated with this document.
    #[serde(rename = "document_number", skip_serializing_if = "Option::is_none")]
    pub document_number: Option<String>,
    /// Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl UpdateIndividualScreeningRequestSearchTerms {
    /// Search terms for editing an individual watchlist screening
    pub fn new() -> UpdateIndividualScreeningRequestSearchTerms {
        UpdateIndividualScreeningRequestSearchTerms {
            watchlist_program_id: None,
            legal_name: None,
            date_of_birth: None,
            document_number: None,
            country: None,
        }
    }
}


