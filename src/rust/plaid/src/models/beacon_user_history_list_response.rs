/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BeaconUserHistoryListResponse : The response schema for `/beacon/user/history/list`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BeaconUserHistoryListResponse {
    #[serde(rename = "beacon_users")]
    pub beacon_users: Vec<crate::models::BeaconUser>,
    /// An identifier that determines which page of results you receive.
    #[serde(rename = "next_cursor", deserialize_with = "Option::deserialize")]
    pub next_cursor: Option<String>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl BeaconUserHistoryListResponse {
    /// The response schema for `/beacon/user/history/list`
    pub fn new(beacon_users: Vec<crate::models::BeaconUser>, next_cursor: Option<String>, request_id: String) -> BeaconUserHistoryListResponse {
        BeaconUserHistoryListResponse {
            beacon_users,
            next_cursor,
            request_id,
        }
    }
}


