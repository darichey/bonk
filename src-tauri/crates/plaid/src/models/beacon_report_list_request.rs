/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BeaconReportListRequest : Request input for listing Beacon Reports



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BeaconReportListRequest {
    /// ID of the associated Beacon User.
    #[serde(rename = "beacon_user_id")]
    pub beacon_user_id: String,
    /// An identifier that determines which page of results you receive.
    #[serde(rename = "cursor", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Option<String>>,
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl BeaconReportListRequest {
    /// Request input for listing Beacon Reports
    pub fn new(beacon_user_id: String) -> BeaconReportListRequest {
        BeaconReportListRequest {
            beacon_user_id,
            cursor: None,
            client_id: None,
            secret: None,
        }
    }
}


