/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IdentityVerificationTemplateReference : The resource ID and version number of the template configuring the behavior of a given identity verification.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IdentityVerificationTemplateReference {
    /// ID of the associated Identity Verification template.
    #[serde(rename = "id")]
    pub id: String,
    /// Version of the associated Identity Verification template.
    #[serde(rename = "version")]
    pub version: i32,
}

impl IdentityVerificationTemplateReference {
    /// The resource ID and version number of the template configuring the behavior of a given identity verification.
    pub fn new(id: String, version: i32) -> IdentityVerificationTemplateReference {
        IdentityVerificationTemplateReference {
            id,
            version,
        }
    }
}


