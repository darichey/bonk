/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenCreateHostedLink : Configuration parameters for Hosted Link. To enable the session for Hosted Link, send this object in the request. It can be empty.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkTokenCreateHostedLink {
    #[serde(rename = "delivery_method", skip_serializing_if = "Option::is_none")]
    pub delivery_method: Option<crate::models::HostedLinkDeliveryMethod>,
    /// URI that Hosted Link will redirect to upon completion of the Link flow. This will only occur in Hosted Link sessions, not in other implementation methods. 
    #[serde(rename = "completion_redirect_uri", skip_serializing_if = "Option::is_none")]
    pub completion_redirect_uri: Option<String>,
    /// How many seconds the link will be valid for. Must be positive. Cannot be longer than 21 days. The default lifetime is 7 days for links delivered by email, 1 day for links delivered via SMS, and 30 minutes for links not sent via Plaid Link delivery. This parameter will override the value of all three link types. 
    #[serde(rename = "url_lifetime_seconds", skip_serializing_if = "Option::is_none")]
    pub url_lifetime_seconds: Option<i32>,
    /// This indicates whether the client is opening hosted Link in a mobile app in an out of process web view (OOPWV) (i.e., an `AsWebAuthenticationSession` / `SFSafariViewController` or Android Custom Tab). 
    #[serde(rename = "is_mobile_app", skip_serializing_if = "Option::is_none")]
    pub is_mobile_app: Option<bool>,
}

impl LinkTokenCreateHostedLink {
    /// Configuration parameters for Hosted Link. To enable the session for Hosted Link, send this object in the request. It can be empty.
    pub fn new() -> LinkTokenCreateHostedLink {
        LinkTokenCreateHostedLink {
            delivery_method: None,
            completion_redirect_uri: None,
            url_lifetime_seconds: None,
            is_mobile_app: None,
        }
    }
}


