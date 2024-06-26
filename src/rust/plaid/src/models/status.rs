/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Status : Documentation not found in the MISMO model viewer and not provided by Freddie Mac.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Status {
    /// Satus Code.
    #[serde(rename = "StatusCode", deserialize_with = "Option::deserialize")]
    pub status_code: Option<String>,
    /// Status Description.
    #[serde(rename = "StatusDescription", deserialize_with = "Option::deserialize")]
    pub status_description: Option<String>,
}

impl Status {
    /// Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    pub fn new(status_code: Option<String>, status_description: Option<String>) -> Status {
        Status {
            status_code,
            status_description,
        }
    }
}


