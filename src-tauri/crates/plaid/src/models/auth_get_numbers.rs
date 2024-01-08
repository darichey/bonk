/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AuthGetNumbers : An object containing identifying numbers used for making electronic transfers to and from the `accounts`. The identifying number type (ACH, EFT, IBAN, or BACS) used will depend on the country of the account. An account may have more than one number type. If a particular identifying number type is not used by any `accounts` for which data has been requested, the array for that type will be empty.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthGetNumbers {
    /// An array of ACH numbers identifying accounts.
    #[serde(rename = "ach")]
    pub ach: Vec<crate::models::NumbersAch>,
    /// An array of EFT numbers identifying accounts.
    #[serde(rename = "eft")]
    pub eft: Vec<crate::models::NumbersEft>,
    /// An array of IBAN numbers identifying accounts.
    #[serde(rename = "international")]
    pub international: Vec<crate::models::NumbersInternational>,
    /// An array of BACS numbers identifying accounts.
    #[serde(rename = "bacs")]
    pub bacs: Vec<crate::models::NumbersBacs>,
}

impl AuthGetNumbers {
    /// An object containing identifying numbers used for making electronic transfers to and from the `accounts`. The identifying number type (ACH, EFT, IBAN, or BACS) used will depend on the country of the account. An account may have more than one number type. If a particular identifying number type is not used by any `accounts` for which data has been requested, the array for that type will be empty.
    pub fn new(ach: Vec<crate::models::NumbersAch>, eft: Vec<crate::models::NumbersEft>, international: Vec<crate::models::NumbersInternational>, bacs: Vec<crate::models::NumbersBacs>) -> AuthGetNumbers {
        AuthGetNumbers {
            ach,
            eft,
            international,
            bacs,
        }
    }
}


