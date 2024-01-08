/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProcessorNumber : An object containing identifying numbers used for making electronic transfers to and from the `account`. The identifying number type (ACH, EFT, IBAN, or BACS) used will depend on the country of the account. An account may have more than one number type. If a particular identifying number type is not used by the `account` for which auth data has been requested, a null value will be returned.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessorNumber {
    #[serde(rename = "ach", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ach: Option<Option<Box<crate::models::NumbersAchNullable>>>,
    #[serde(rename = "eft", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub eft: Option<Option<Box<crate::models::NumbersEftNullable>>>,
    #[serde(rename = "international", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub international: Option<Option<Box<crate::models::NumbersInternationalNullable>>>,
    #[serde(rename = "bacs", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bacs: Option<Option<Box<crate::models::NumbersBacsNullable>>>,
}

impl ProcessorNumber {
    /// An object containing identifying numbers used for making electronic transfers to and from the `account`. The identifying number type (ACH, EFT, IBAN, or BACS) used will depend on the country of the account. An account may have more than one number type. If a particular identifying number type is not used by the `account` for which auth data has been requested, a null value will be returned.
    pub fn new() -> ProcessorNumber {
        ProcessorNumber {
            ach: None,
            eft: None,
            international: None,
            bacs: None,
        }
    }
}


