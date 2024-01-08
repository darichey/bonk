/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PartnerEndCustomerOAuthInstitutionEnvironments : Registration statuses by environment.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartnerEndCustomerOAuthInstitutionEnvironments {
    #[serde(rename = "development", skip_serializing_if = "Option::is_none")]
    pub development: Option<crate::models::PartnerEndCustomerOAuthInstitutionApplicationStatus>,
    #[serde(rename = "production", skip_serializing_if = "Option::is_none")]
    pub production: Option<crate::models::PartnerEndCustomerOAuthInstitutionApplicationStatus>,
}

impl PartnerEndCustomerOAuthInstitutionEnvironments {
    /// Registration statuses by environment.
    pub fn new() -> PartnerEndCustomerOAuthInstitutionEnvironments {
        PartnerEndCustomerOAuthInstitutionEnvironments {
            development: None,
            production: None,
        }
    }
}


