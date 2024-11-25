/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BeaconUserRequestDepositoryAccount : Depository account information for the associated user.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BeaconUserRequestDepositoryAccount {
    /// Must be a valid US Bank Account Number
    #[serde(rename = "account_number")]
    pub account_number: String,
    /// The routing number of the account.
    #[serde(rename = "routing_number")]
    pub routing_number: String,
}

impl BeaconUserRequestDepositoryAccount {
    /// Depository account information for the associated user.
    pub fn new(account_number: String, routing_number: String) -> BeaconUserRequestDepositoryAccount {
        BeaconUserRequestDepositoryAccount {
            account_number,
            routing_number,
        }
    }
}


