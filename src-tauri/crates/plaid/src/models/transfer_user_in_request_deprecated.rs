/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferUserInRequestDeprecated : The legal name and other information for the account holder.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferUserInRequestDeprecated {
    /// The user's legal name.
    #[serde(rename = "legal_name", skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    /// The user's phone number.
    #[serde(rename = "phone_number", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// The user's email address.
    #[serde(rename = "email_address", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Box<crate::models::TransferUserAddressInRequest>>,
}

impl TransferUserInRequestDeprecated {
    /// The legal name and other information for the account holder.
    pub fn new() -> TransferUserInRequestDeprecated {
        TransferUserInRequestDeprecated {
            legal_name: None,
            phone_number: None,
            email_address: None,
            address: None,
        }
    }
}


