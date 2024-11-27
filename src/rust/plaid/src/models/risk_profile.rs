/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RiskProfile : RiskProfile is deprecated, use `ruleset` instead.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RiskProfile {
    /// The key of the risk profile used for this transaction.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The evaluated outcome for this transaction. You can configure a list of outcomes, such as \"accept\", \"review\", and \"decline\" using the Signal dashboard located within the Plaid Dashboard.
    #[serde(rename = "outcome", skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
}

impl RiskProfile {
    /// RiskProfile is deprecated, use `ruleset` instead.
    pub fn new() -> RiskProfile {
        RiskProfile {
            key: None,
            outcome: None,
        }
    }
}

