/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenCreateRequestStatements : Specifies options for initializing Link for use with the Statements product.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestStatements {
    /// The start date for statements, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) “YYYY-MM-DD” format, e.g. \"2020-10-30\". If no value is provided, this will default to 3 months prior to the current date.
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// The end date for statements, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) “YYYY-MM-DD” format, e.g. \"2020-10-30\". If no value is provided, this will default to the current date. You can request up to two years of data.
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

impl LinkTokenCreateRequestStatements {
    /// Specifies options for initializing Link for use with the Statements product.
    pub fn new() -> LinkTokenCreateRequestStatements {
        LinkTokenCreateRequestStatements {
            start_date: None,
            end_date: None,
        }
    }
}


