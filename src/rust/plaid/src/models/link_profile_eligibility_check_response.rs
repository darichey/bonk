/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkProfileEligibilityCheckResponse : LinkProfileEligibilityCheckResponse defines the response schema for `/link/profile/eligibility/check`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkProfileEligibilityCheckResponse {
    /// Indicates whether Plaid has a profile matching the customer's eligibility requirements for this user
    #[serde(rename = "profile_matches")]
    pub profile_matches: bool,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl LinkProfileEligibilityCheckResponse {
    /// LinkProfileEligibilityCheckResponse defines the response schema for `/link/profile/eligibility/check`
    pub fn new(profile_matches: bool, request_id: String) -> LinkProfileEligibilityCheckResponse {
        LinkProfileEligibilityCheckResponse {
            profile_matches,
            request_id,
        }
    }
}


