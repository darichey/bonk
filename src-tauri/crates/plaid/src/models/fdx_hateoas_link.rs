/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FdxHateoasLink : REST application constraint (Hypermedia As The Engine Of Application State)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FdxHateoasLink {
    /// URL to invoke the action on the resource
    #[serde(rename = "href")]
    pub href: String,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<crate::models::FdxHateoasLinkAction>,
    /// Relation of this link to its containing entity, as defined by and with many example relation values at [IETF RFC5988](https://datatracker.ietf.org/doc/html/rfc5988)
    #[serde(rename = "rel", skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
    /// Content-types that can be used in the Accept header
    #[serde(rename = "types", skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<crate::models::FdxContentTypes>>,
}

impl FdxHateoasLink {
    /// REST application constraint (Hypermedia As The Engine Of Application State)
    pub fn new(href: String) -> FdxHateoasLink {
        FdxHateoasLink {
            href,
            action: None,
            rel: None,
            types: None,
        }
    }
}


