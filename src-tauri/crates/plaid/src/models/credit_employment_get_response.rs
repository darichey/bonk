/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditEmploymentGetResponse : CreditEmploymentGetResponse defines the response schema for `/credit/employment/get`.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditEmploymentGetResponse {
    /// Array of employment items.
    #[serde(rename = "items")]
    pub items: Vec<crate::models::CreditEmploymentItem>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl CreditEmploymentGetResponse {
    /// CreditEmploymentGetResponse defines the response schema for `/credit/employment/get`.
    pub fn new(items: Vec<crate::models::CreditEmploymentItem>, request_id: String) -> CreditEmploymentGetResponse {
        CreditEmploymentGetResponse {
            items,
            request_id,
        }
    }
}


