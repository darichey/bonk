/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NumbersBacs : Identifying information for transferring money to or from a UK bank account via BACS.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumbersBacs {
    /// The Plaid account ID associated with the account numbers
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The BACS account number for the account
    #[serde(rename = "account")]
    pub account: String,
    /// The BACS sort code for the account
    #[serde(rename = "sort_code")]
    pub sort_code: String,
}

impl NumbersBacs {
    /// Identifying information for transferring money to or from a UK bank account via BACS.
    pub fn new(account_id: String, account: String, sort_code: String) -> NumbersBacs {
        NumbersBacs {
            account_id,
            account,
            sort_code,
        }
    }
}


