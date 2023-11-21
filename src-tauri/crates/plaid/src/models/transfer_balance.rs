/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferBalance : Information about the balance held with Plaid.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferBalance {
    /// The amount of this balance available for use (decimal string with two digits of precision e.g. \"10.00\").
    #[serde(rename = "available")]
    pub available: String,
    /// The available balance, plus amount of pending funds that in processing (decimal string with two digits of precision e.g. \"10.00\").
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    pub current: Option<String>,
    #[serde(rename = "type")]
    pub r#type: crate::models::TransferBalanceType,
}

impl TransferBalance {
    /// Information about the balance held with Plaid.
    pub fn new(available: String, r#type: crate::models::TransferBalanceType) -> TransferBalance {
        TransferBalance {
            available,
            current: None,
            r#type,
        }
    }
}

