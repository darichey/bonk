/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DepositSwitchTargetAccount : The deposit switch destination account



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DepositSwitchTargetAccount {
    /// Account number for deposit switch destination
    #[serde(rename = "account_number")]
    pub account_number: String,
    /// Routing number for deposit switch destination
    #[serde(rename = "routing_number")]
    pub routing_number: String,
    /// The name of the deposit switch destination account, as it will be displayed to the end user in the Deposit Switch interface. It is not required to match the name used in online banking.
    #[serde(rename = "account_name")]
    pub account_name: String,
    /// The account subtype of the account, either `checking` or `savings`.
    #[serde(rename = "account_subtype")]
    pub account_subtype: AccountSubtype,
}

impl DepositSwitchTargetAccount {
    /// The deposit switch destination account
    pub fn new(account_number: String, routing_number: String, account_name: String, account_subtype: AccountSubtype) -> DepositSwitchTargetAccount {
        DepositSwitchTargetAccount {
            account_number,
            routing_number,
            account_name,
            account_subtype,
        }
    }
}

/// The account subtype of the account, either `checking` or `savings`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountSubtype {
    #[serde(rename = "checking")]
    Checking,
    #[serde(rename = "savings")]
    Savings,
}

impl Default for AccountSubtype {
    fn default() -> AccountSubtype {
        Self::Checking
    }
}

