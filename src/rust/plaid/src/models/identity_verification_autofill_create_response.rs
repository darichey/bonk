/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IdentityVerificationAutofillCreateResponse : Autofill represents unverified customer information. This needs to be confirmed by the customer before using.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IdentityVerificationAutofillCreateResponse {
    #[serde(rename = "status")]
    pub status: crate::models::IdentityVerificationAutofillStatus,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<crate::models::IdentityVerificationAutofillUserData>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl IdentityVerificationAutofillCreateResponse {
    /// Autofill represents unverified customer information. This needs to be confirmed by the customer before using.
    pub fn new(status: crate::models::IdentityVerificationAutofillStatus, user: Option<crate::models::IdentityVerificationAutofillUserData>, request_id: String) -> IdentityVerificationAutofillCreateResponse {
        IdentityVerificationAutofillCreateResponse {
            status,
            user,
            request_id,
        }
    }
}


