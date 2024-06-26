/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// VerificationRefreshStatus : The verification refresh status. One of the following:  `\"VERIFICATION_REFRESH_STATUS_USER_PRESENCE_REQUIRED\"` User presence is required to refresh an income verification. `\"VERIFICATION_REFRESH_SUCCESSFUL\"` The income verification refresh was successful. `\"VERIFICATION_REFRESH_NOT_FOUND\"` No new data was found after the income verification refresh.

/// The verification refresh status. One of the following:  `\"VERIFICATION_REFRESH_STATUS_USER_PRESENCE_REQUIRED\"` User presence is required to refresh an income verification. `\"VERIFICATION_REFRESH_SUCCESSFUL\"` The income verification refresh was successful. `\"VERIFICATION_REFRESH_NOT_FOUND\"` No new data was found after the income verification refresh.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VerificationRefreshStatus {
    #[serde(rename = "VERIFICATION_REFRESH_STATUS_USER_PRESENCE_REQUIRED")]
    StatusUserPresenceRequired,
    #[serde(rename = "VERIFICATION_REFRESH_SUCCESSFUL")]
    Successful,
    #[serde(rename = "VERIFICATION_REFRESH_NOT_FOUND")]
    NotFound,

}

impl ToString for VerificationRefreshStatus {
    fn to_string(&self) -> String {
        match self {
            Self::StatusUserPresenceRequired => String::from("VERIFICATION_REFRESH_STATUS_USER_PRESENCE_REQUIRED"),
            Self::Successful => String::from("VERIFICATION_REFRESH_SUCCESSFUL"),
            Self::NotFound => String::from("VERIFICATION_REFRESH_NOT_FOUND"),
        }
    }
}

impl Default for VerificationRefreshStatus {
    fn default() -> VerificationRefreshStatus {
        Self::StatusUserPresenceRequired
    }
}




