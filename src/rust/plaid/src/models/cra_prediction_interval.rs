/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraPredictionInterval : The object containing prediction interval data.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CraPredictionInterval {
    /// The lower bound of the predicted attribute for the given probability.
    #[serde(rename = "lower_bound", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub lower_bound: Option<Option<f32>>,
    /// The upper bound of the predicted attribute for the given probability.
    #[serde(rename = "upper_bound", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub upper_bound: Option<Option<f32>>,
    /// The probability of the actual value of the attribute falling within the upper and lower bound. This is a percentage represented as a value between 0 and 1.
    #[serde(rename = "probability", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub probability: Option<Option<f32>>,
}

impl CraPredictionInterval {
    /// The object containing prediction interval data.
    pub fn new() -> CraPredictionInterval {
        CraPredictionInterval {
            lower_bound: None,
            upper_bound: None,
            probability: None,
        }
    }
}


