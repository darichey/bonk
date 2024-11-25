/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserAccountIdentityAddress : The user's address.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserAccountIdentityAddress {
    /// The full city name
    #[serde(rename = "city", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub city: Option<Option<String>>,
    /// The region or state. Example: `\"NC\"`
    #[serde(rename = "region", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub region: Option<Option<String>>,
    /// The full street address Example: `\"564 Main Street, APT 15\"`
    #[serde(rename = "street", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub street: Option<Option<String>>,
    /// The second line street address
    #[serde(rename = "street2", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub street2: Option<Option<String>>,
    /// The postal code. In API versions 2018-05-22 and earlier, this field is called `zip`.
    #[serde(rename = "postal_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<Option<String>>,
    /// The ISO 3166-1 alpha-2 country code
    #[serde(rename = "country", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub country: Option<Option<String>>,
}

impl UserAccountIdentityAddress {
    /// The user's address.
    pub fn new() -> UserAccountIdentityAddress {
        UserAccountIdentityAddress {
            city: None,
            region: None,
            street: None,
            street2: None,
            postal_code: None,
            country: None,
        }
    }
}


