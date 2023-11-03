/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationAddress : The optional address of the payment recipient's bank account. Required by most institutions outside of the UK.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentInitiationAddress {
    /// An array of length 1-2 representing the street address where the recipient is located. Maximum of 70 characters.
    #[serde(rename = "street")]
    pub street: Vec<String>,
    /// The city where the recipient is located. Maximum of 35 characters.
    #[serde(rename = "city")]
    pub city: String,
    /// The postal code where the recipient is located. Maximum of 16 characters.
    #[serde(rename = "postal_code")]
    pub postal_code: String,
    /// The ISO 3166-1 alpha-2 country code where the recipient is located.
    #[serde(rename = "country")]
    pub country: String,
}

impl PaymentInitiationAddress {
    /// The optional address of the payment recipient's bank account. Required by most institutions outside of the UK.
    pub fn new(street: Vec<String>, city: String, postal_code: String, country: String) -> PaymentInitiationAddress {
        PaymentInitiationAddress {
            street,
            city,
            postal_code,
            country,
        }
    }
}


