/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BeaconUserName : The full name for a given Beacon User.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BeaconUserName {
    /// A string with at least one non-whitespace character, with a max length of 100 characters.
    #[serde(rename = "given_name")]
    pub given_name: String,
    /// A string with at least one non-whitespace character, with a max length of 100 characters.
    #[serde(rename = "family_name")]
    pub family_name: String,
}

impl BeaconUserName {
    /// The full name for a given Beacon User.
    pub fn new(given_name: String, family_name: String) -> BeaconUserName {
        BeaconUserName {
            given_name,
            family_name,
        }
    }
}


