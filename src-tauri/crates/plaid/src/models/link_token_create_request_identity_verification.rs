/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenCreateRequestIdentityVerification : Specifies option for initializing Link for use with the Identity Verification product.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestIdentityVerification {
    /// ID of the associated Identity Verification template.
    #[serde(rename = "template_id")]
    pub template_id: String,
    /// A flag specifying whether the end user has already agreed to a privacy policy specifying that their data will be shared with Plaid for verification purposes.  If `gave_consent` is set to `true`, the `accept_tos` step will be marked as `skipped` and the end user's session will start at the next step requirement.
    #[serde(rename = "consent", skip_serializing_if = "Option::is_none")]
    pub consent: Option<bool>,
    /// A flag specifying whether the end user has already agreed to a privacy policy specifying that their data will be shared with Plaid for verification purposes.  If `gave_consent` is set to `true`, the `accept_tos` step will be marked as `skipped` and the end user's session will start at the next step requirement.
    #[serde(rename = "gave_consent", skip_serializing_if = "Option::is_none")]
    pub gave_consent: Option<bool>,
}

impl LinkTokenCreateRequestIdentityVerification {
    /// Specifies option for initializing Link for use with the Identity Verification product.
    pub fn new(template_id: String) -> LinkTokenCreateRequestIdentityVerification {
        LinkTokenCreateRequestIdentityVerification {
            template_id,
            consent: None,
            gave_consent: None,
        }
    }
}


