/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DateRange : A date range with a start and end date



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DateRange {
    /// A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(rename = "beginning")]
    pub beginning: String,
    /// A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(rename = "ending")]
    pub ending: String,
}

impl DateRange {
    /// A date range with a start and end date
    pub fn new(beginning: String, ending: String) -> DateRange {
        DateRange {
            beginning,
            ending,
        }
    }
}


