/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BankTransferBalanceGetRequest : Defines the request schema for `/bank_transfer/balance/get`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BankTransferBalanceGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// If multiple origination accounts are available, `origination_account_id` must be used to specify the account for which balance will be returned.
    #[serde(rename = "origination_account_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub origination_account_id: Option<Option<String>>,
}

impl BankTransferBalanceGetRequest {
    /// Defines the request schema for `/bank_transfer/balance/get`
    pub fn new() -> BankTransferBalanceGetRequest {
        BankTransferBalanceGetRequest {
            client_id: None,
            secret: None,
            origination_account_id: None,
        }
    }
}


