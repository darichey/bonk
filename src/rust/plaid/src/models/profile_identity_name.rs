/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProfileIdentityName : ProfileIdentityName defines the user's first name and last name.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProfileIdentityName {
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

impl ProfileIdentityName {
    /// ProfileIdentityName defines the user's first name and last name.
    pub fn new() -> ProfileIdentityName {
        ProfileIdentityName {
            first_name: None,
            last_name: None,
        }
    }
}


