/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserThirdPartyTokenRemoveResponse : UserThirdPartyTokenCreateResponse defines the response schema for `/user/third_party_token/remove`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserThirdPartyTokenRemoveResponse {
    /// `true` if the third-party user token was successfully removed.
    #[serde(rename = "removed")]
    pub removed: bool,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl UserThirdPartyTokenRemoveResponse {
    /// UserThirdPartyTokenCreateResponse defines the response schema for `/user/third_party_token/remove`
    pub fn new(removed: bool, request_id: String) -> UserThirdPartyTokenRemoveResponse {
        UserThirdPartyTokenRemoveResponse {
            removed,
            request_id,
        }
    }
}

