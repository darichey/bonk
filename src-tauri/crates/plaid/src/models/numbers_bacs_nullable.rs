/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NumbersBacsNullable : Identifying information for transferring money to or from a UK bank account via BACS.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NumbersBacsNullable {
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

impl NumbersBacsNullable {
    /// Identifying information for transferring money to or from a UK bank account via BACS.
    pub fn new(account_id: String, account: String, sort_code: String) -> NumbersBacsNullable {
        NumbersBacsNullable {
            account_id,
            account,
            sort_code,
        }
    }
}


