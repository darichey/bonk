/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BeaconUserRequestData : A Beacon User's data which is used to check against duplicate records and the Beacon Fraud Network.  In order to create a Beacon User, in addition to the `name`, _either_ the `date_of_birth` _or_ the `depository_accounts` field must be provided.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BeaconUserRequestData {
    /// A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(rename = "date_of_birth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(rename = "name")]
    pub name: crate::models::BeaconUserName,
    #[serde(rename = "address", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address: Option<Option<crate::models::BeaconUserRequestAddress>>,
    /// A valid email address. Must not have leading or trailing spaces and address must be RFC compliant. For more information, see [RFC 3696](https://datatracker.ietf.org/doc/html/rfc3696).
    #[serde(rename = "email_address", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// A phone number in E.164 format.
    #[serde(rename = "phone_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<Option<String>>,
    #[serde(rename = "id_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id_number: Option<Option<crate::models::BeaconUserIdNumber>>,
    /// An IPv4 or IPV6 address.
    #[serde(rename = "ip_address", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<Option<String>>,
    /// Provide a list of bank accounts that are associated with this Beacon User. These accounts will be scanned across the Beacon Network and used to find duplicate records.  Note: These accounts will not have Bank Account Insights. To receive Bank Account Insights please supply `access_tokens`.
    #[serde(rename = "depository_accounts", skip_serializing_if = "Option::is_none")]
    pub depository_accounts: Option<Vec<crate::models::BeaconUserRequestDepositoryAccount>>,
}

impl BeaconUserRequestData {
    /// A Beacon User's data which is used to check against duplicate records and the Beacon Fraud Network.  In order to create a Beacon User, in addition to the `name`, _either_ the `date_of_birth` _or_ the `depository_accounts` field must be provided.
    pub fn new(name: crate::models::BeaconUserName) -> BeaconUserRequestData {
        BeaconUserRequestData {
            date_of_birth: None,
            name,
            address: None,
            email_address: None,
            phone_number: None,
            id_number: None,
            ip_address: None,
            depository_accounts: None,
        }
    }
}


