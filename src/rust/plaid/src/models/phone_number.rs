/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PhoneNumber : A phone number



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhoneNumber {
    /// The phone number.
    #[serde(rename = "data")]
    pub data: String,
    /// When `true`, identifies the phone number as the primary number on an account.
    #[serde(rename = "primary")]
    pub primary: bool,
    /// The type of phone number.
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl PhoneNumber {
    /// A phone number
    pub fn new(data: String, primary: bool, r#type: Type) -> PhoneNumber {
        PhoneNumber {
            data,
            primary,
            r#type,
        }
    }
}

/// The type of phone number.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "work")]
    Work,
    #[serde(rename = "office")]
    Office,
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "mobile1")]
    Mobile1,
    #[serde(rename = "other")]
    Other,
}

impl Default for Type {
    fn default() -> Type {
        Self::Home
    }
}

