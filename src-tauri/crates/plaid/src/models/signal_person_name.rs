/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SignalPersonName : The user's legal name



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignalPersonName {
    /// The user's name prefix (e.g. \"Mr.\")
    #[serde(rename = "prefix", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<Option<String>>,
    /// The user's given name. If the user has a one-word name, it should be provided in this field.
    #[serde(rename = "given_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub given_name: Option<Option<String>>,
    /// The user's middle name
    #[serde(rename = "middle_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<Option<String>>,
    /// The user's family name / surname
    #[serde(rename = "family_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub family_name: Option<Option<String>>,
    /// The user's name suffix (e.g. \"II\")
    #[serde(rename = "suffix", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<Option<String>>,
}

impl SignalPersonName {
    /// The user's legal name
    pub fn new() -> SignalPersonName {
        SignalPersonName {
            prefix: None,
            given_name: None,
            middle_name: None,
            family_name: None,
            suffix: None,
        }
    }
}


