/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PlaidCheckScore : The results of the Plaid Check score



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PlaidCheckScore {
    /// The score returned by the Plaid Check Score model.
    #[serde(rename = "score", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub score: Option<Option<f32>>,
    /// The reasons for an individual having risk according to the Plaid Check score.
    #[serde(rename = "reason_codes", skip_serializing_if = "Option::is_none")]
    pub reason_codes: Option<Vec<String>>,
    /// Human-readable description of why the Plaid Check score could not be computed.
    #[serde(rename = "error_reason", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<Option<String>>,
}

impl PlaidCheckScore {
    /// The results of the Plaid Check score
    pub fn new() -> PlaidCheckScore {
        PlaidCheckScore {
            score: None,
            reason_codes: None,
            error_reason: None,
        }
    }
}


