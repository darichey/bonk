/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferAuthorizationGuaranteeDecisionRationale : The rationale for Plaid's decision to not guarantee a transfer. Will be `null` unless `guarantee_decision` is `NOT_GUARANTEED`.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferAuthorizationGuaranteeDecisionRationale {
    #[serde(rename = "code")]
    pub code: crate::models::TransferAuthorizationGuaranteeDecisionRationaleCode,
    /// A human-readable description of why the transfer cannot be guaranteed.
    #[serde(rename = "description")]
    pub description: String,
}

impl TransferAuthorizationGuaranteeDecisionRationale {
    /// The rationale for Plaid's decision to not guarantee a transfer. Will be `null` unless `guarantee_decision` is `NOT_GUARANTEED`.
    pub fn new(code: crate::models::TransferAuthorizationGuaranteeDecisionRationaleCode, description: String) -> TransferAuthorizationGuaranteeDecisionRationale {
        TransferAuthorizationGuaranteeDecisionRationale {
            code,
            description,
        }
    }
}


