/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PartnerCustomerCreateResponse : Response schema for `/partner/customer/create`.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartnerCustomerCreateResponse {
    #[serde(rename = "end_customer", skip_serializing_if = "Option::is_none")]
    pub end_customer: Option<Box<crate::models::PartnerEndCustomerWithSecrets>>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl PartnerCustomerCreateResponse {
    /// Response schema for `/partner/customer/create`.
    pub fn new() -> PartnerCustomerCreateResponse {
        PartnerCustomerCreateResponse {
            end_customer: None,
            request_id: None,
        }
    }
}


