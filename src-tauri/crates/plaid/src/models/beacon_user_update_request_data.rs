/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BeaconUserUpdateRequestData : A subset of a Beacon User's data which is used to patch the existing identity data associated with a Beacon User. At least one field must be provided,.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeaconUserUpdateRequestData {
    /// A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(rename = "date_of_birth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<crate::models::BeaconUserNameNullable>>,
    #[serde(rename = "address", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address: Option<Option<crate::models::BeaconUserRequestAddressNullable>>,
    /// A valid email address.
    #[serde(rename = "email_address", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// A phone number in E.164 format.
    #[serde(rename = "phone_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<Option<String>>,
    #[serde(rename = "id_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id_number: Option<Option<crate::models::BeaconUserIdNumber>>,
    /// An IPv4 or IPV6 address.
    #[serde(rename = "ip_address", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<Option<String>>,
}

impl BeaconUserUpdateRequestData {
    /// A subset of a Beacon User's data which is used to patch the existing identity data associated with a Beacon User. At least one field must be provided,.
    pub fn new() -> BeaconUserUpdateRequestData {
        BeaconUserUpdateRequestData {
            date_of_birth: None,
            name: None,
            address: None,
            email_address: None,
            phone_number: None,
            id_number: None,
            ip_address: None,
        }
    }
}


