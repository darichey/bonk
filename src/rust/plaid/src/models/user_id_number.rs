/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserIdNumber : ID number submitted by the user, currently used only for the Identity Verification product. If the user has not submitted this data yet, this field will be `null`. Otherwise, both fields are guaranteed to be filled.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserIdNumber {
    /// Value of identity document value typed in by user. Alpha-numeric, with all formatting characters stripped.
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::IdNumberType,
}

impl UserIdNumber {
    /// ID number submitted by the user, currently used only for the Identity Verification product. If the user has not submitted this data yet, this field will be `null`. Otherwise, both fields are guaranteed to be filled.
    pub fn new(value: String, r#type: crate::models::IdNumberType) -> UserIdNumber {
        UserIdNumber {
            value,
            r#type,
        }
    }
}


