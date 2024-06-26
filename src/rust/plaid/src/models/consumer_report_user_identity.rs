/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConsumerReportUserIdentity : ConsumerReportUserIdentity defines the user identity data collected for consumer report purpose. This field is required to be set if you later use the created user for consumer report purpose.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsumerReportUserIdentity {
    /// The user's first name
    #[serde(rename = "first_name")]
    pub first_name: String,
    /// The user's last name
    #[serde(rename = "last_name")]
    pub last_name: String,
    /// The user's phone numbers. The format of phone number will be validated and for better normalization, it is expected to be in E.164 format +{countrycode}{number}, for example `+14151234567`.
    #[serde(rename = "phone_numbers")]
    pub phone_numbers: Vec<String>,
    /// The user's emails
    #[serde(rename = "emails")]
    pub emails: Vec<String>,
    #[serde(rename = "primary_address")]
    pub primary_address: crate::models::AddressData,
}

impl ConsumerReportUserIdentity {
    /// ConsumerReportUserIdentity defines the user identity data collected for consumer report purpose. This field is required to be set if you later use the created user for consumer report purpose.
    pub fn new(first_name: String, last_name: String, phone_numbers: Vec<String>, emails: Vec<String>, primary_address: crate::models::AddressData) -> ConsumerReportUserIdentity {
        ConsumerReportUserIdentity {
            first_name,
            last_name,
            phone_numbers,
            emails,
            primary_address,
        }
    }
}


