/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferSweepListRequest : Defines the request schema for `/transfer/sweep/list`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferSweepListRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The start datetime of sweeps to return (RFC 3339 format).
    #[serde(rename = "start_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Option<String>>,
    /// The end datetime of sweeps to return (RFC 3339 format).
    #[serde(rename = "end_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Option<String>>,
    /// The maximum number of sweeps to return.
    #[serde(rename = "count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub count: Option<Option<i32>>,
    /// The number of sweeps to skip before returning results.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Filter sweeps to only those with the specified amount.
    #[serde(rename = "amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub amount: Option<Option<String>>,
    #[serde(rename = "status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<crate::models::SweepStatus>>,
    /// Filter sweeps to only those with the specified originator client.
    #[serde(rename = "originator_client_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub originator_client_id: Option<Option<String>>,
    /// Filter sweeps to only those with the specified `funding_account_id`.
    #[serde(rename = "funding_account_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub funding_account_id: Option<Option<String>>,
    /// Filter sweeps to only those with the included `transfer_id`.
    #[serde(rename = "transfer_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<Option<String>>,
    #[serde(rename = "trigger", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Option<crate::models::SweepTrigger>>,
}

impl TransferSweepListRequest {
    /// Defines the request schema for `/transfer/sweep/list`
    pub fn new() -> TransferSweepListRequest {
        TransferSweepListRequest {
            client_id: None,
            secret: None,
            start_date: None,
            end_date: None,
            count: None,
            offset: None,
            amount: None,
            status: None,
            originator_client_id: None,
            funding_account_id: None,
            transfer_id: None,
            trigger: None,
        }
    }
}


