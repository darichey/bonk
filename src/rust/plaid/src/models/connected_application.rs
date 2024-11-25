/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConnectedApplication : Describes the connected application for a particular end user.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConnectedApplication {
    /// This field will map to the application ID that is returned from /item/application/list, or provided to the institution in an oauth redirect.
    #[serde(rename = "application_id")]
    pub application_id: String,
    /// The name of the application
    #[serde(rename = "name")]
    pub name: String,
    /// A human-readable name of the application for display purposes
    #[serde(rename = "display_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<Option<String>>,
    /// A URL that links to the application logo image.
    #[serde(rename = "logo_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<Option<String>>,
    /// The URL for the application's website
    #[serde(rename = "application_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub application_url: Option<Option<String>>,
    /// A string provided by the connected app stating why they use their respective enabled products.
    #[serde(rename = "reason_for_access", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub reason_for_access: Option<Option<String>>,
    /// The date this application was linked in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC.
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "scopes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Option<Box<crate::models::ScopesNullable>>>,
}

impl ConnectedApplication {
    /// Describes the connected application for a particular end user.
    pub fn new(application_id: String, name: String, created_at: String) -> ConnectedApplication {
        ConnectedApplication {
            application_id,
            name,
            display_name: None,
            logo_url: None,
            application_url: None,
            reason_for_access: None,
            created_at,
            scopes: None,
        }
    }
}


