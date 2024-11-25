/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PartnerEndCustomerWithSecretsAllOf {
    #[serde(rename = "secrets", skip_serializing_if = "Option::is_none")]
    pub secrets: Option<crate::models::PartnerEndCustomerSecrets>,
}

impl PartnerEndCustomerWithSecretsAllOf {
    pub fn new() -> PartnerEndCustomerWithSecretsAllOf {
        PartnerEndCustomerWithSecretsAllOf {
            secrets: None,
        }
    }
}


