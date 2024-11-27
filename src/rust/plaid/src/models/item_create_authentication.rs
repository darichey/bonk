/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemCreateAuthentication : Enum representing the entity authenticating the user.

/// Enum representing the entity authenticating the user.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ItemCreateAuthentication {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "DATA_PARTNER")]
    DataPartner,
    #[serde(rename = "PLAID")]
    Plaid,

}

impl ToString for ItemCreateAuthentication {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("UNKNOWN"),
            Self::DataPartner => String::from("DATA_PARTNER"),
            Self::Plaid => String::from("PLAID"),
        }
    }
}

impl Default for ItemCreateAuthentication {
    fn default() -> ItemCreateAuthentication {
        Self::Unknown
    }
}



