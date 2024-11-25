/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AuthSupportedMethods : Metadata specifically related to which auth methods an institution supports.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AuthSupportedMethods {
    /// Indicates if instant auth is supported.
    #[serde(rename = "instant_auth")]
    pub instant_auth: bool,
    /// Indicates if instant match is supported.
    #[serde(rename = "instant_match")]
    pub instant_match: bool,
    /// Indicates if automated microdeposits are supported.
    #[serde(rename = "automated_micro_deposits")]
    pub automated_micro_deposits: bool,
    /// Indicates if instant microdeposits are supported.
    #[serde(rename = "instant_micro_deposits")]
    pub instant_micro_deposits: bool,
}

impl AuthSupportedMethods {
    /// Metadata specifically related to which auth methods an institution supports.
    pub fn new(instant_auth: bool, instant_match: bool, automated_micro_deposits: bool, instant_micro_deposits: bool) -> AuthSupportedMethods {
        AuthSupportedMethods {
            instant_auth,
            instant_match,
            automated_micro_deposits,
            instant_micro_deposits,
        }
    }
}


