/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferLedgerGetRequest : Defines the request schema for `/transfer/ledger/get`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferLedgerGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Specify which ledger balance to get. Customers can find a list of `ledger_id`s in the Accounts page of your Plaid Dashboard. If this field is left blank, this will default to id of the default ledger balance.
    #[serde(rename = "ledger_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ledger_id: Option<Option<String>>,
    /// Client ID of the end customer.
    #[serde(rename = "originator_client_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub originator_client_id: Option<Option<String>>,
}

impl TransferLedgerGetRequest {
    /// Defines the request schema for `/transfer/ledger/get`
    pub fn new() -> TransferLedgerGetRequest {
        TransferLedgerGetRequest {
            client_id: None,
            secret: None,
            ledger_id: None,
            originator_client_id: None,
        }
    }
}


