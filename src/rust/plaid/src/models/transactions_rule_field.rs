/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsRuleField : Transaction field for which the rule is defined.

/// Transaction field for which the rule is defined.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionsRuleField {
    #[serde(rename = "TRANSACTION_ID")]
    TransactionId,
    #[serde(rename = "NAME")]
    Name,

}

impl ToString for TransactionsRuleField {
    fn to_string(&self) -> String {
        match self {
            Self::TransactionId => String::from("TRANSACTION_ID"),
            Self::Name => String::from("NAME"),
        }
    }
}

impl Default for TransactionsRuleField {
    fn default() -> TransactionsRuleField {
        Self::TransactionId
    }
}




