/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemApplicationScopesUpdateResponse : ItemApplicationScopesUpdateResponse defines the response schema for `/item/application/scopes/update`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemApplicationScopesUpdateResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl ItemApplicationScopesUpdateResponse {
    /// ItemApplicationScopesUpdateResponse defines the response schema for `/item/application/scopes/update`
    pub fn new(request_id: String) -> ItemApplicationScopesUpdateResponse {
        ItemApplicationScopesUpdateResponse {
            request_id,
        }
    }
}


