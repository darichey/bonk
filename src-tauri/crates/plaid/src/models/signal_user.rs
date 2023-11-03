/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SignalUser : Details about the end user initiating the transaction (i.e., the account holder).



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SignalUser {
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<Box<crate::models::SignalPersonName>>>,
    /// The user's phone number, in E.164 format: +{countrycode}{number}. For example: \"+14151234567\"
    #[serde(rename = "phone_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<Option<String>>,
    /// The user's email address.
    #[serde(rename = "email_address", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<Option<String>>,
    #[serde(rename = "address", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address: Option<Option<crate::models::SignalAddressData>>,
}

impl SignalUser {
    /// Details about the end user initiating the transaction (i.e., the account holder).
    pub fn new() -> SignalUser {
        SignalUser {
            name: None,
            phone_number: None,
            email_address: None,
            address: None,
        }
    }
}


