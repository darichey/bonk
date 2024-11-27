/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// NumbersAch : Identifying information for transferring money to or from a US account via ACH or wire transfer.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumbersAch {
    /// The Plaid account ID associated with the account numbers
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The ACH account number for the account.  At certain institutions, including Chase and PNC, you will receive \"tokenized\" routing and account numbers, which are not the user's actual account and routing numbers. For important details on how this may impact your integration and on how to avoid fraud, user confusion, and ACH returns, see [Tokenized account numbers](https://plaid.com/docs/auth/#tokenized-account-numbers).
    #[serde(rename = "account")]
    pub account: String,
    /// The ACH routing number for the account. This may be a tokenized routing number. For more information, see [Tokenized account numbers](https://plaid.com/docs/auth/#tokenized-account-numbers).
    #[serde(rename = "routing")]
    pub routing: String,
    /// The wire transfer routing number for the account, if available
    #[serde(rename = "wire_routing", deserialize_with = "Option::deserialize")]
    pub wire_routing: Option<String>,
    /// Whether the account supports ACH transfers into the account
    #[serde(rename = "can_transfer_in", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub can_transfer_in: Option<Option<bool>>,
    /// Whether the account supports ACH transfers out of the account
    #[serde(rename = "can_transfer_out", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub can_transfer_out: Option<Option<bool>>,
}

impl NumbersAch {
    /// Identifying information for transferring money to or from a US account via ACH or wire transfer.
    pub fn new(account_id: String, account: String, routing: String, wire_routing: Option<String>) -> NumbersAch {
        NumbersAch {
            account_id,
            account,
            routing,
            wire_routing,
            can_transfer_in: None,
            can_transfer_out: None,
        }
    }
}

