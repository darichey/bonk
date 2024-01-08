/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenCreateHostedLink : Configuration parameters for Hosted Link (beta). Only available for participants in the Hosted Link beta program.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokenCreateHostedLink {
    #[serde(rename = "delivery_method", skip_serializing_if = "Option::is_none")]
    pub delivery_method: Option<crate::models::HostedLinkDeliveryMethod>,
    /// URI that Hosted Link will redirect to upon completion of the Link flow. This will only occur in Hosted Link sessions, not in other implementation methods. 
    #[serde(rename = "completion_redirect_uri", skip_serializing_if = "Option::is_none")]
    pub completion_redirect_uri: Option<String>,
    /// How many seconds the link will be valid for. Must be positive. Cannot be longer than 21 days. The default lifetime is 4 hours. 
    #[serde(rename = "url_lifetime_seconds", skip_serializing_if = "Option::is_none")]
    pub url_lifetime_seconds: Option<i32>,
}

impl LinkTokenCreateHostedLink {
    /// Configuration parameters for Hosted Link (beta). Only available for participants in the Hosted Link beta program.
    pub fn new() -> LinkTokenCreateHostedLink {
        LinkTokenCreateHostedLink {
            delivery_method: None,
            completion_redirect_uri: None,
            url_lifetime_seconds: None,
        }
    }
}


