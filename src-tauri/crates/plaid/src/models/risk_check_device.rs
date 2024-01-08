/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RiskCheckDevice : Result summary object specifying values for `device` attributes of risk check.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RiskCheckDevice {
    #[serde(rename = "ip_proxy_type", deserialize_with = "Option::deserialize")]
    pub ip_proxy_type: Option<crate::models::ProxyType>,
    /// Count of spam lists the IP address is associated with if known.
    #[serde(rename = "ip_spam_list_count", deserialize_with = "Option::deserialize")]
    pub ip_spam_list_count: Option<i32>,
    /// UTC offset of the timezone associated with the IP address.
    #[serde(rename = "ip_timezone_offset", deserialize_with = "Option::deserialize")]
    pub ip_timezone_offset: Option<String>,
}

impl RiskCheckDevice {
    /// Result summary object specifying values for `device` attributes of risk check.
    pub fn new(ip_proxy_type: Option<crate::models::ProxyType>, ip_spam_list_count: Option<i32>, ip_timezone_offset: Option<String>) -> RiskCheckDevice {
        RiskCheckDevice {
            ip_proxy_type,
            ip_spam_list_count,
            ip_timezone_offset,
        }
    }
}


