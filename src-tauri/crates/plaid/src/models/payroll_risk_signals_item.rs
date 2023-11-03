/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PayrollRiskSignalsItem : Object containing fraud risk data pertaining to the Item linked as part of the verification.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PayrollRiskSignalsItem {
    /// The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// Array of payroll income document authenticity data retrieved for each of the user's accounts.
    #[serde(rename = "verification_risk_signals")]
    pub verification_risk_signals: Vec<crate::models::DocumentRiskSignalsObject>,
}

impl PayrollRiskSignalsItem {
    /// Object containing fraud risk data pertaining to the Item linked as part of the verification.
    pub fn new(item_id: String, verification_risk_signals: Vec<crate::models::DocumentRiskSignalsObject>) -> PayrollRiskSignalsItem {
        PayrollRiskSignalsItem {
            item_id,
            verification_risk_signals,
        }
    }
}


