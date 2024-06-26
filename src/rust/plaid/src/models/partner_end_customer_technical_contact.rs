/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PartnerEndCustomerTechnicalContact : The technical contact for the end customer. Defaults to partner's technical contact if omitted.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartnerEndCustomerTechnicalContact {
    #[serde(rename = "given_name", skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(rename = "family_name", skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl PartnerEndCustomerTechnicalContact {
    /// The technical contact for the end customer. Defaults to partner's technical contact if omitted.
    pub fn new() -> PartnerEndCustomerTechnicalContact {
        PartnerEndCustomerTechnicalContact {
            given_name: None,
            family_name: None,
            email: None,
        }
    }
}


