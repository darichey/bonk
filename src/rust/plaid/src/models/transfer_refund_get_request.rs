/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferRefundGetRequest : Defines the request schema for `/transfer/refund/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferRefundGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Plaid’s unique identifier for a refund.
    #[serde(rename = "refund_id")]
    pub refund_id: String,
}

impl TransferRefundGetRequest {
    /// Defines the request schema for `/transfer/refund/get`
    pub fn new(refund_id: String) -> TransferRefundGetRequest {
        TransferRefundGetRequest {
            client_id: None,
            secret: None,
            refund_id,
        }
    }
}


