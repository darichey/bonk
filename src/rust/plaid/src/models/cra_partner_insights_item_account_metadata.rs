/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraPartnerInsightsItemAccountMetadata : An object containing metadata about the extracted account.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CraPartnerInsightsItemAccountMetadata {
    /// The date of the earliest extracted transaction, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (\"yyyy-mm-dd\").
    #[serde(rename = "start_date", deserialize_with = "Option::deserialize")]
    pub start_date: Option<String>,
    /// The date of the most recent extracted transaction, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (\"yyyy-mm-dd\").
    #[serde(rename = "end_date", deserialize_with = "Option::deserialize")]
    pub end_date: Option<String>,
}

impl CraPartnerInsightsItemAccountMetadata {
    /// An object containing metadata about the extracted account.
    pub fn new(start_date: Option<String>, end_date: Option<String>) -> CraPartnerInsightsItemAccountMetadata {
        CraPartnerInsightsItemAccountMetadata {
            start_date,
            end_date,
        }
    }
}

