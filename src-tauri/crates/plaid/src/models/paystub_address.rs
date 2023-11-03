/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaystubAddress : Address on the paystub



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaystubAddress {
    /// The full city name.
    #[serde(rename = "city", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub city: Option<Option<String>>,
    /// The ISO 3166-1 alpha-2 country code.
    #[serde(rename = "country", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub country: Option<Option<String>>,
    /// The postal code of the address.
    #[serde(rename = "postal_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<Option<String>>,
    /// The region or state Example: `\"NC\"`
    #[serde(rename = "region", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub region: Option<Option<String>>,
    /// The full street address.
    #[serde(rename = "street", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub street: Option<Option<String>>,
    /// Street address line 1.
    #[serde(rename = "line1", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub line1: Option<Option<String>>,
    /// Street address line 2.
    #[serde(rename = "line2", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub line2: Option<Option<String>>,
    /// The region or state Example: `\"NC\"`
    #[serde(rename = "state_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub state_code: Option<Option<String>>,
}

impl PaystubAddress {
    /// Address on the paystub
    pub fn new() -> PaystubAddress {
        PaystubAddress {
            city: None,
            country: None,
            postal_code: None,
            region: None,
            street: None,
            line1: None,
            line2: None,
            state_code: None,
        }
    }
}


