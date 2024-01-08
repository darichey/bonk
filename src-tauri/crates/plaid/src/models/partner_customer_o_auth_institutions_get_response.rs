/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PartnerCustomerOAuthInstitutionsGetResponse : Response schema for `/partner/customer/oauth_institutions/get`.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartnerCustomerOAuthInstitutionsGetResponse {
    #[serde(rename = "flowdown_status", skip_serializing_if = "Option::is_none")]
    pub flowdown_status: Option<crate::models::PartnerEndCustomerFlowdownStatus>,
    #[serde(rename = "questionnaire_status", skip_serializing_if = "Option::is_none")]
    pub questionnaire_status: Option<crate::models::PartnerEndCustomerQuestionnaireStatus>,
    /// The OAuth institutions with which the end customer's application is being registered.
    #[serde(rename = "institutions", skip_serializing_if = "Option::is_none")]
    pub institutions: Option<Vec<crate::models::PartnerEndCustomerOAuthInstitution>>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl PartnerCustomerOAuthInstitutionsGetResponse {
    /// Response schema for `/partner/customer/oauth_institutions/get`.
    pub fn new() -> PartnerCustomerOAuthInstitutionsGetResponse {
        PartnerCustomerOAuthInstitutionsGetResponse {
            flowdown_status: None,
            questionnaire_status: None,
            institutions: None,
            request_id: None,
        }
    }
}


