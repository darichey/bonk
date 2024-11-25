/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraBankIncomeGetResponse : CraBankIncomeGetResponse defines the response schema for `/cra/bank_income/get`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CraBankIncomeGetResponse {
    #[serde(rename = "bank_income", skip_serializing_if = "Option::is_none")]
    pub bank_income: Option<Vec<crate::models::CraBankIncome>>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl CraBankIncomeGetResponse {
    /// CraBankIncomeGetResponse defines the response schema for `/cra/bank_income/get`.
    pub fn new(request_id: String) -> CraBankIncomeGetResponse {
        CraBankIncomeGetResponse {
            bank_income: None,
            request_id,
        }
    }
}


