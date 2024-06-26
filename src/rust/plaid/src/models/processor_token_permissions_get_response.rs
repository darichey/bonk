/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProcessorTokenPermissionsGetResponse : ProcessorTokenPermissionsGetResponse defines the response schema for `/processor/token/permissions/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessorTokenPermissionsGetResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
    /// A list of products the processor token should have access to. An empty list means that the processor has access to all available products, including future products.
    #[serde(rename = "products")]
    pub products: Vec<crate::models::Products>,
}

impl ProcessorTokenPermissionsGetResponse {
    /// ProcessorTokenPermissionsGetResponse defines the response schema for `/processor/token/permissions/get`
    pub fn new(request_id: String, products: Vec<crate::models::Products>) -> ProcessorTokenPermissionsGetResponse {
        ProcessorTokenPermissionsGetResponse {
            request_id,
            products,
        }
    }
}


