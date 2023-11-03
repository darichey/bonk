/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferLedgerDepositRequest : Defines the request schema for `/transfer/ledger/deposit`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferLedgerDepositRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Client ID of the customer that owns the Ledger balance. This is so Plaid knows which of your customers to payout or collect funds. Only applicable for [Platform customers](https://plaid.com/docs/transfer/application/#originators-vs-platforms). Do not include if you’re paying out to yourself.
    #[serde(rename = "originator_client_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub originator_client_id: Option<Option<String>>,
    /// Specify which funding account linked to this Plaid Ledger to use. Customers can find a list of `funding_account_id`s in the Accounts page of your Plaid Dashboard, under the \"Account ID\" column. If this field is left blank, this will default to the default `funding_account_id` specified during onboarding. If an `originator_client_id` is specified, the `funding_account_id` must belong to the specified originator, and if `funding_account_id` is left blank, the originator's default `funding_account_id` will be used.
    #[serde(rename = "funding_account_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub funding_account_id: Option<Option<String>>,
    /// A positive amount of how much will be deposited into ledger (decimal string with two digits of precision e.g. \"5.50\").
    #[serde(rename = "amount")]
    pub amount: String,
    /// The description of the deposit that will be passed to the receiving bank (up to 10 characters). Note that banks utilize this field differently, and may or may not show it on the bank statement.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// A unique key provided by the client, per unique ledger deposit. Maximum of 50 characters.  The API supports idempotency for safely retrying the request without accidentally performing the same operation twice. For example, if a request to create a ledger deposit fails due to a network connection error, you can retry the request with the same idempotency key to guarantee that only a single deposit is created.
    #[serde(rename = "idempotency_key")]
    pub idempotency_key: String,
    #[serde(rename = "network")]
    pub network: crate::models::TransferAchNetwork,
}

impl TransferLedgerDepositRequest {
    /// Defines the request schema for `/transfer/ledger/deposit`
    pub fn new(amount: String, idempotency_key: String, network: crate::models::TransferAchNetwork) -> TransferLedgerDepositRequest {
        TransferLedgerDepositRequest {
            client_id: None,
            secret: None,
            originator_client_id: None,
            funding_account_id: None,
            amount,
            description: None,
            idempotency_key,
            network,
        }
    }
}


