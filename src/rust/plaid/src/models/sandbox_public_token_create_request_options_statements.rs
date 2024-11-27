/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxPublicTokenCreateRequestOptionsStatements : An optional set of parameters corresponding to statements options.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SandboxPublicTokenCreateRequestOptionsStatements {
    /// The earliest date for which to fetch statements history. Dates should be formatted as YYYY-MM-DD.
    #[serde(rename = "start_date")]
    pub start_date: String,
    /// The most recent date for which to fetch statements history. Dates should be formatted as YYYY-MM-DD.
    #[serde(rename = "end_date")]
    pub end_date: String,
}

impl SandboxPublicTokenCreateRequestOptionsStatements {
    /// An optional set of parameters corresponding to statements options.
    pub fn new(start_date: String, end_date: String) -> SandboxPublicTokenCreateRequestOptionsStatements {
        SandboxPublicTokenCreateRequestOptionsStatements {
            start_date,
            end_date,
        }
    }
}

