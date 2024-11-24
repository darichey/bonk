/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraLoansRegisterResponse : CraLoansRegisterResponse defines the response schema for `/cra/loans/register`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CraLoansRegisterResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl CraLoansRegisterResponse {
    /// CraLoansRegisterResponse defines the response schema for `/cra/loans/register`.
    pub fn new(request_id: String) -> CraLoansRegisterResponse {
        CraLoansRegisterResponse {
            request_id,
        }
    }
}


