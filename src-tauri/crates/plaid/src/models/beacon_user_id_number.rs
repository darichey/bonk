/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BeaconUserIdNumber : The ID number associated with a Beacon User.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BeaconUserIdNumber {
    /// Value of identity document value typed in by user. Alpha-numeric, with all formatting characters stripped.
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::IdNumberType,
}

impl BeaconUserIdNumber {
    /// The ID number associated with a Beacon User.
    pub fn new(value: String, r#type: crate::models::IdNumberType) -> BeaconUserIdNumber {
        BeaconUserIdNumber {
            value,
            r#type,
        }
    }
}


