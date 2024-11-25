/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkSessionSuccess : An object representing an [onSuccess](https://plaid.com/docs/link/web/#onsuccess) callback from Link. This object has been deprecated in favor of the newer [`results.item_add_result`](https://plaid.com/docs/api/link/#link-token-get-response-link-sessions-results-item-add-results), which can support multiple public tokens in a single Link session.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkSessionSuccess {
    /// Displayed once a user has successfully linked their Item.
    #[serde(rename = "public_token")]
    pub public_token: String,
    #[serde(rename = "metadata", deserialize_with = "Option::deserialize")]
    pub metadata: Option<Box<crate::models::LinkSessionSuccessMetadata>>,
}

impl LinkSessionSuccess {
    /// An object representing an [onSuccess](https://plaid.com/docs/link/web/#onsuccess) callback from Link. This object has been deprecated in favor of the newer [`results.item_add_result`](https://plaid.com/docs/api/link/#link-token-get-response-link-sessions-results-item-add-results), which can support multiple public tokens in a single Link session.
    pub fn new(public_token: String, metadata: Option<crate::models::LinkSessionSuccessMetadata>) -> LinkSessionSuccess {
        LinkSessionSuccess {
            public_token,
            metadata: if let Some(x) = metadata {Some(Box::new(x))} else {None},
        }
    }
}


