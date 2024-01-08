/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PhoneType : An enum indicating whether a phone number is a phone line or a fax line.

/// An enum indicating whether a phone number is a phone line or a fax line.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PhoneType {
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "fax")]
    Fax,

}

impl ToString for PhoneType {
    fn to_string(&self) -> String {
        match self {
            Self::Phone => String::from("phone"),
            Self::Fax => String::from("fax"),
        }
    }
}

impl Default for PhoneType {
    fn default() -> PhoneType {
        Self::Phone
    }
}




