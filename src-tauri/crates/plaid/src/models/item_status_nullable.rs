/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemStatusNullable : Information about the last successful and failed transactions update for the Item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemStatusNullable {
    #[serde(rename = "investments", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub investments: Option<Option<crate::models::ItemStatusInvestments>>,
    #[serde(rename = "transactions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Option<crate::models::ItemStatusTransactions>>,
    #[serde(rename = "last_webhook", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_webhook: Option<Option<crate::models::ItemStatusLastWebhook>>,
}

impl ItemStatusNullable {
    /// Information about the last successful and failed transactions update for the Item.
    pub fn new() -> ItemStatusNullable {
        ItemStatusNullable {
            investments: None,
            transactions: None,
            last_webhook: None,
        }
    }
}


