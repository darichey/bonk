/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BeaconUserDepositoryAccount : Depository account information for the associated user.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BeaconUserDepositoryAccount {
    /// The last 2-4 numeric characters of this account’s account number.
    #[serde(rename = "account_mask")]
    pub account_mask: String,
    /// The routing number of the account.
    #[serde(rename = "routing_number")]
    pub routing_number: String,
    /// An ISO8601 formatted timestamp.
    #[serde(rename = "added_at")]
    pub added_at: String,
}

impl BeaconUserDepositoryAccount {
    /// Depository account information for the associated user.
    pub fn new(account_mask: String, routing_number: String, added_at: String) -> BeaconUserDepositoryAccount {
        BeaconUserDepositoryAccount {
            account_mask,
            routing_number,
            added_at,
        }
    }
}


