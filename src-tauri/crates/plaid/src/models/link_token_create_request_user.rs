/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenCreateRequestUser : An object specifying information about the end user who will be linking their account.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestUser {
    /// A unique ID representing the end user. Typically this will be a user ID number from your application. Personally identifiable information, such as an email address or phone number, should not be used in the `client_user_id`. It is currently used as a means of searching logs for the given user in the Plaid Dashboard.
    #[serde(rename = "client_user_id")]
    pub client_user_id: String,
    /// The user's full legal name, used for [micro-deposit based verification flows](https://plaid.com/docs/auth/coverage/). For a small number of customers on legacy flows, providing this field is required to enable micro-deposit-based flows. For all other customers, this field is optional, but providing the user's name in this field when using micro-deposit-based verification will enable certain risk checks and can reduce micro-deposit fraud.
    #[serde(rename = "legal_name", skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<crate::models::LinkTokenCreateRequestUserName>>,
    /// The user's phone number in [E.164](https://en.wikipedia.org/wiki/E.164) format. This field is optional, but required to enable the [returning user experience](https://plaid.com/docs/link/returning-user). Can also be used to prefill Link fields when used with Identity Verification.
    #[serde(rename = "phone_number", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// The date and time the phone number was verified in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDThh:mm:ssZ`). This was previously an optional field used in the [returning user experience](https://plaid.com/docs/link/returning-user). This field is no longer required to enable the returning user experience.   Only pass a verification time for a phone number that you have verified. If you have performed verification but don’t have the time, you may supply a signal value of the start of the UNIX epoch.   Example: `2020-01-01T00:00:00Z` 
    #[serde(rename = "phone_number_verified_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone_number_verified_time: Option<Option<String>>,
    /// The user's email address. This field is optional, but required to enable the [pre-authenticated returning user flow](https://plaid.com/docs/link/returning-user/#pre-authenticated-rux). Can also be used to prefill Link fields when used with Identity Verification.
    #[serde(rename = "email_address", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// The date and time the email address was verified in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDThh:mm:ssZ`). This was previously an optional field used in the [returning user experience](https://plaid.com/docs/link/returning-user). This field is no longer required to enable the returning user experience.   Only pass a verification time for an email address that you have verified. If you have performed verification but don’t have the time, you may supply a signal value of the start of the UNIX epoch.   Example: `2020-01-01T00:00:00Z`
    #[serde(rename = "email_address_verified_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub email_address_verified_time: Option<Option<String>>,
    /// Deprecated and not currently used, use the `id_number` field instead.
    #[serde(rename = "ssn", skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
    /// To be provided in the format \"yyyy-mm-dd\". Can be used to prefill Link fields when used with Identity Verification.
    #[serde(rename = "date_of_birth", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<Option<String>>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Box<crate::models::LinkTokenCreateRequestUserAddress>>,
    #[serde(rename = "id_number", skip_serializing_if = "Option::is_none")]
    pub id_number: Option<Box<crate::models::LinkTokenCreateRequestUserIdNumber>>,
}

impl LinkTokenCreateRequestUser {
    /// An object specifying information about the end user who will be linking their account.
    pub fn new(client_user_id: String) -> LinkTokenCreateRequestUser {
        LinkTokenCreateRequestUser {
            client_user_id,
            legal_name: None,
            name: None,
            phone_number: None,
            phone_number_verified_time: None,
            email_address: None,
            email_address_verified_time: None,
            ssn: None,
            date_of_birth: None,
            address: None,
            id_number: None,
        }
    }
}


