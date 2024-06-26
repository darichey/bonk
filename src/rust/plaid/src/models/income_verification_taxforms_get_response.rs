/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IncomeVerificationTaxformsGetResponse : IncomeVerificationTaxformsGetResponse defines the response schema for `/income/verification/taxforms/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncomeVerificationTaxformsGetResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "document_metadata")]
    pub document_metadata: Vec<crate::models::DocumentMetadata>,
    /// A list of forms.
    #[serde(rename = "taxforms")]
    pub taxforms: Vec<crate::models::Taxform>,
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error: Option<Option<crate::models::PlaidError>>,
}

impl IncomeVerificationTaxformsGetResponse {
    /// IncomeVerificationTaxformsGetResponse defines the response schema for `/income/verification/taxforms/get`
    pub fn new(document_metadata: Vec<crate::models::DocumentMetadata>, taxforms: Vec<crate::models::Taxform>) -> IncomeVerificationTaxformsGetResponse {
        IncomeVerificationTaxformsGetResponse {
            request_id: None,
            document_metadata,
            taxforms,
            error: None,
        }
    }
}


