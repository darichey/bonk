/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AssetReportUser : The user object allows you to provide additional information about the user to be appended to the Asset Report. All fields are optional. The `first_name`, `last_name`, and `ssn` fields are required if you would like the Report to be eligible for Fannie Mae’s Day 1 Certainty™ program.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetReportUser {
    /// An identifier you determine and submit for the user.
    #[serde(rename = "client_user_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub client_user_id: Option<Option<String>>,
    /// The user's first name. Required for the Fannie Mae Day 1 Certainty™ program.
    #[serde(rename = "first_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Option<String>>,
    /// The user's middle name
    #[serde(rename = "middle_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<Option<String>>,
    /// The user's last name.  Required for the Fannie Mae Day 1 Certainty™ program.
    #[serde(rename = "last_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Option<String>>,
    /// The user's Social Security Number. Required for the Fannie Mae Day 1 Certainty™ program.  Format: \"ddd-dd-dddd\"
    #[serde(rename = "ssn", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ssn: Option<Option<String>>,
    /// The user's phone number, in E.164 format: +{countrycode}{number}. For example: \"+14151234567\". Phone numbers provided in other formats will be parsed on a best-effort basis.
    #[serde(rename = "phone_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<Option<String>>,
    /// The user's email address.
    #[serde(rename = "email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
}

impl AssetReportUser {
    /// The user object allows you to provide additional information about the user to be appended to the Asset Report. All fields are optional. The `first_name`, `last_name`, and `ssn` fields are required if you would like the Report to be eligible for Fannie Mae’s Day 1 Certainty™ program.
    pub fn new() -> AssetReportUser {
        AssetReportUser {
            client_user_id: None,
            first_name: None,
            middle_name: None,
            last_name: None,
            ssn: None,
            phone_number: None,
            email: None,
        }
    }
}


