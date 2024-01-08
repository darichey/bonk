/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferListRequest : Defines the request schema for `/transfer/list`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferListRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The start datetime of transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    #[serde(rename = "start_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Option<String>>,
    /// The end datetime of transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    #[serde(rename = "end_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Option<String>>,
    /// The maximum number of transfers to return.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// The number of transfers to skip before returning results.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Filter transfers to only those originated through the specified origination account.
    #[serde(rename = "origination_account_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub origination_account_id: Option<Option<String>>,
    /// Filter transfers to only those with the specified originator client.
    #[serde(rename = "originator_client_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub originator_client_id: Option<Option<String>>,
    /// Filter transfers to only those with the specified `funding_account_id`.
    #[serde(rename = "funding_account_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub funding_account_id: Option<Option<String>>,
}

impl TransferListRequest {
    /// Defines the request schema for `/transfer/list`
    pub fn new() -> TransferListRequest {
        TransferListRequest {
            client_id: None,
            secret: None,
            start_date: None,
            end_date: None,
            count: None,
            offset: None,
            origination_account_id: None,
            originator_client_id: None,
            funding_account_id: None,
        }
    }
}


