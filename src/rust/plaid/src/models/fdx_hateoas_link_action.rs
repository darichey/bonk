/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FdxHateoasLinkAction : HTTP Method to use for the request

/// HTTP Method to use for the request
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FdxHateoasLinkAction {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "PUT")]
    Put,

}

impl ToString for FdxHateoasLinkAction {
    fn to_string(&self) -> String {
        match self {
            Self::Get => String::from("GET"),
            Self::Post => String::from("POST"),
            Self::Patch => String::from("PATCH"),
            Self::Delete => String::from("DELETE"),
            Self::Put => String::from("PUT"),
        }
    }
}

impl Default for FdxHateoasLinkAction {
    fn default() -> FdxHateoasLinkAction {
        Self::Get
    }
}




