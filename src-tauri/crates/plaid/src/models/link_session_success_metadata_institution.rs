/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkSessionSuccessMetadataInstitution : An institution object. If the Item was created via Same-Day micro-deposit verification, will be `null`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkSessionSuccessMetadataInstitution {
    /// The full institution name, such as `'Wells Fargo'`
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The Plaid institution identifier
    #[serde(rename = "institution_id", skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
}

impl LinkSessionSuccessMetadataInstitution {
    /// An institution object. If the Item was created via Same-Day micro-deposit verification, will be `null`.
    pub fn new() -> LinkSessionSuccessMetadataInstitution {
        LinkSessionSuccessMetadataInstitution {
            name: None,
            institution_id: None,
        }
    }
}

