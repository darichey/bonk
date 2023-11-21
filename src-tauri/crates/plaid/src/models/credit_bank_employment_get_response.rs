/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankEmploymentGetResponse : CreditBankEmploymentGetResponse defines the response schema for `/beta/credit/v1/bank_employment/get`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditBankEmploymentGetResponse {
    /// Bank Employment data. Each entry in the array will be a distinct bank employment report.
    #[serde(rename = "bank_employment_reports")]
    pub bank_employment_reports: Vec<crate::models::CreditBankEmploymentReport>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl CreditBankEmploymentGetResponse {
    /// CreditBankEmploymentGetResponse defines the response schema for `/beta/credit/v1/bank_employment/get`.
    pub fn new(bank_employment_reports: Vec<crate::models::CreditBankEmploymentReport>, request_id: String) -> CreditBankEmploymentGetResponse {
        CreditBankEmploymentGetResponse {
            bank_employment_reports,
            request_id,
        }
    }
}

