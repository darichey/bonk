/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferDevice : Information about the device being used to initiate the authorization.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferDevice {
    /// The IP address of the device being used to initiate the authorization.
    #[serde(rename = "ip_address")]
    pub ip_address: String,
    /// The user agent of the device being used to initiate the authorization.
    #[serde(rename = "user_agent")]
    pub user_agent: String,
}

impl TransferDevice {
    /// Information about the device being used to initiate the authorization.
    pub fn new(ip_address: String, user_agent: String) -> TransferDevice {
        TransferDevice {
            ip_address,
            user_agent,
        }
    }
}


