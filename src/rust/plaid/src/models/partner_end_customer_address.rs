/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PartnerEndCustomerAddress : The end customer's address.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PartnerEndCustomerAddress {
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "street", skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "postal_code", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// ISO-3166-1 alpha-2 country code standard.
    #[serde(rename = "country_code", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
}

impl PartnerEndCustomerAddress {
    /// The end customer's address.
    pub fn new() -> PartnerEndCustomerAddress {
        PartnerEndCustomerAddress {
            city: None,
            street: None,
            region: None,
            postal_code: None,
            country_code: None,
        }
    }
}


