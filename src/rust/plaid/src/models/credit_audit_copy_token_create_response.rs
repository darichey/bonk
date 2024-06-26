/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditAuditCopyTokenCreateResponse : CreditAuditCopyTokenCreateResponse defines the response schema for `/credit/audit_copy_token/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditAuditCopyTokenCreateResponse {
    /// A token that can be shared with a third party auditor, which allows them to fetch the Asset Reports attached to the token. This token should be stored securely.
    #[serde(rename = "audit_copy_token")]
    pub audit_copy_token: String,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl CreditAuditCopyTokenCreateResponse {
    /// CreditAuditCopyTokenCreateResponse defines the response schema for `/credit/audit_copy_token/get`
    pub fn new(audit_copy_token: String, request_id: String) -> CreditAuditCopyTokenCreateResponse {
        CreditAuditCopyTokenCreateResponse {
            audit_copy_token,
            request_id,
        }
    }
}


