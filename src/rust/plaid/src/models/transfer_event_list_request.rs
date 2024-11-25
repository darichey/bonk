/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferEventListRequest : Defines the request schema for `/transfer/event/list`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferEventListRequest {
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
    /// Plaid’s unique identifier for a transfer.
    #[serde(rename = "transfer_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<Option<String>>,
    /// The account ID to get events for all transactions to/from an account.
    #[serde(rename = "account_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Option<String>>,
    #[serde(rename = "transfer_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<Option<crate::models::TransferEventListTransferType>>,
    /// Filter events by event type.
    #[serde(rename = "event_types", skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<crate::models::TransferEventType>>,
    /// Plaid’s unique identifier for a sweep.
    #[serde(rename = "sweep_id", skip_serializing_if = "Option::is_none")]
    pub sweep_id: Option<String>,
    /// The maximum number of transfer events to return. If the number of events matching the above parameters is greater than `count`, the most recent events will be returned.
    #[serde(rename = "count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub count: Option<Option<i32>>,
    /// The offset into the list of transfer events. When `count`=25 and `offset`=0, the first 25 events will be returned. When `count`=25 and `offset`=25, the next 25 events will be returned.
    #[serde(rename = "offset", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub offset: Option<Option<i32>>,
    /// The origination account ID to get events for transfers from a specific origination account.
    #[serde(rename = "origination_account_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub origination_account_id: Option<Option<String>>,
    /// Filter transfer events to only those with the specified originator client.
    #[serde(rename = "originator_client_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub originator_client_id: Option<Option<String>>,
    /// Filter transfer events to only those with the specified `funding_account_id`.
    #[serde(rename = "funding_account_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub funding_account_id: Option<Option<String>>,
}

impl TransferEventListRequest {
    /// Defines the request schema for `/transfer/event/list`
    pub fn new() -> TransferEventListRequest {
        TransferEventListRequest {
            client_id: None,
            secret: None,
            start_date: None,
            end_date: None,
            transfer_id: None,
            account_id: None,
            transfer_type: None,
            event_types: None,
            sweep_id: None,
            count: None,
            offset: None,
            origination_account_id: None,
            originator_client_id: None,
            funding_account_id: None,
        }
    }
}


