/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DetectedAccount : A possible account detected to be associated with a transaction user.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DetectedAccount {
    /// The detected account type (depository, credit, loan, investment etc.).
    #[serde(rename = "account_type", deserialize_with = "Option::deserialize")]
    pub account_type: Option<String>,
    /// The detected subtype of the account, based on the transactions to/from the institution.
    #[serde(rename = "account_subtype", deserialize_with = "Option::deserialize")]
    pub account_subtype: Option<String>,
    /// The number of transactions associated with this detected account type at this financial institution.
    #[serde(rename = "transaction_count")]
    pub transaction_count: i32,
    /// The date of the oldest transaction associated with this detected account type at this financial institution.
    #[serde(rename = "oldest_transaction_date", skip_serializing_if = "Option::is_none")]
    pub oldest_transaction_date: Option<String>,
    /// The date of the newest transaction associated with this detected account type at this financial institution.
    #[serde(rename = "newest_transaction_date", skip_serializing_if = "Option::is_none")]
    pub newest_transaction_date: Option<String>,
    /// Amount of the most recent transaction associated with this detected account type at this financial institution.
    #[serde(rename = "newest_transaction_amount", skip_serializing_if = "Option::is_none")]
    pub newest_transaction_amount: Option<f64>,
    /// Sum of outflow amounts associated with this detected account type at this financial institution.
    #[serde(rename = "total_outflows")]
    pub total_outflows: f64,
    /// Sum of inflow amounts associated with this detected account type at this financial institution.
    #[serde(rename = "total_inflows")]
    pub total_inflows: f64,
}

impl DetectedAccount {
    /// A possible account detected to be associated with a transaction user.
    pub fn new(account_type: Option<String>, account_subtype: Option<String>, transaction_count: i32, total_outflows: f64, total_inflows: f64) -> DetectedAccount {
        DetectedAccount {
            account_type,
            account_subtype,
            transaction_count,
            oldest_transaction_date: None,
            newest_transaction_date: None,
            newest_transaction_amount: None,
            total_outflows,
            total_inflows,
        }
    }
}


