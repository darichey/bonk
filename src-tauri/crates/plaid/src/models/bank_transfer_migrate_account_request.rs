/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BankTransferMigrateAccountRequest : Defines the request schema for `/bank_transfer/migrate_account`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BankTransferMigrateAccountRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The user's account number.
    #[serde(rename = "account_number")]
    pub account_number: String,
    /// The user's routing number.
    #[serde(rename = "routing_number")]
    pub routing_number: String,
    /// The user's wire transfer routing number. This is the ABA number; for some institutions, this may differ from the ACH number used in `routing_number`.
    #[serde(rename = "wire_routing_number", skip_serializing_if = "Option::is_none")]
    pub wire_routing_number: Option<String>,
    /// The type of the bank account (`checking` or `savings`).
    #[serde(rename = "account_type")]
    pub account_type: String,
}

impl BankTransferMigrateAccountRequest {
    /// Defines the request schema for `/bank_transfer/migrate_account`
    pub fn new(account_number: String, routing_number: String, account_type: String) -> BankTransferMigrateAccountRequest {
        BankTransferMigrateAccountRequest {
            client_id: None,
            secret: None,
            account_number,
            routing_number,
            wire_routing_number: None,
            account_type,
        }
    }
}


