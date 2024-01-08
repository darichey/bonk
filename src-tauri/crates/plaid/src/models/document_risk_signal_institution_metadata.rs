/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DocumentRiskSignalInstitutionMetadata : An object which contains additional metadata about the institution used to compute the verification attribute



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentRiskSignalInstitutionMetadata {
    /// The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(rename = "item_id")]
    pub item_id: String,
}

impl DocumentRiskSignalInstitutionMetadata {
    /// An object which contains additional metadata about the institution used to compute the verification attribute
    pub fn new(item_id: String) -> DocumentRiskSignalInstitutionMetadata {
        DocumentRiskSignalInstitutionMetadata {
            item_id,
        }
    }
}


