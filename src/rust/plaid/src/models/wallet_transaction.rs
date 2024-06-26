/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WalletTransaction : The transaction details



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WalletTransaction {
    /// A unique ID identifying the transaction
    #[serde(rename = "transaction_id")]
    pub transaction_id: String,
    /// The EMI (E-Money Institution) wallet that this payment is associated with, if any. This wallet is used as an intermediary account to enable Plaid to reconcile the settlement of funds for Payment Initiation requests.
    #[serde(rename = "wallet_id")]
    pub wallet_id: String,
    /// A reference for the transaction
    #[serde(rename = "reference")]
    pub reference: String,
    /// The type of the transaction. The supported transaction types that are returned are: `BANK_TRANSFER:` a transaction which credits an e-wallet through an external bank transfer.  `PAYOUT:` a transaction which debits an e-wallet by disbursing funds to a counterparty.  `PIS_PAY_IN:` a payment which credits an e-wallet through Plaid's Payment Initiation Services (PIS) APIs. For more information see the [Payment Initiation endpoints](https://plaid.com/docs/api/products/payment-initiation/).  `REFUND:` a transaction which debits an e-wallet by refunding a previously initiated payment made through Plaid's [PIS APIs](https://plaid.com/docs/api/products/payment-initiation/).  `FUNDS_SWEEP`: an automated transaction which debits funds from an e-wallet to a designated client-owned account.  `RETURN`: an automated transaction where a debit transaction was reversed and money moved back to originating account.
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "scheme", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scheme: Option<Option<crate::models::WalletPaymentScheme>>,
    #[serde(rename = "amount")]
    pub amount: crate::models::WalletTransactionAmount,
    #[serde(rename = "counterparty")]
    pub counterparty: crate::models::WalletTransactionCounterparty,
    #[serde(rename = "status")]
    pub status: crate::models::WalletTransactionStatus,
    /// Timestamp when the transaction was created, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The date and time of the last time the `status` was updated, in IS0 8601 format
    #[serde(rename = "last_status_update")]
    pub last_status_update: String,
    /// The payment id that this transaction is associated with, if any. This is present only for transaction types `PIS_PAY_IN` and `REFUND`.
    #[serde(rename = "payment_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<Option<String>>,
    #[serde(rename = "failure_reason", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<Option<crate::models::WalletTransactionFailureReason>>,
}

impl WalletTransaction {
    /// The transaction details
    pub fn new(transaction_id: String, wallet_id: String, reference: String, r#type: Type, amount: crate::models::WalletTransactionAmount, counterparty: crate::models::WalletTransactionCounterparty, status: crate::models::WalletTransactionStatus, created_at: String, last_status_update: String) -> WalletTransaction {
        WalletTransaction {
            transaction_id,
            wallet_id,
            reference,
            r#type,
            scheme: None,
            amount,
            counterparty,
            status,
            created_at,
            last_status_update,
            payment_id: None,
            failure_reason: None,
        }
    }
}

/// The type of the transaction. The supported transaction types that are returned are: `BANK_TRANSFER:` a transaction which credits an e-wallet through an external bank transfer.  `PAYOUT:` a transaction which debits an e-wallet by disbursing funds to a counterparty.  `PIS_PAY_IN:` a payment which credits an e-wallet through Plaid's Payment Initiation Services (PIS) APIs. For more information see the [Payment Initiation endpoints](https://plaid.com/docs/api/products/payment-initiation/).  `REFUND:` a transaction which debits an e-wallet by refunding a previously initiated payment made through Plaid's [PIS APIs](https://plaid.com/docs/api/products/payment-initiation/).  `FUNDS_SWEEP`: an automated transaction which debits funds from an e-wallet to a designated client-owned account.  `RETURN`: an automated transaction where a debit transaction was reversed and money moved back to originating account.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "BANK_TRANSFER")]
    BankTransfer,
    #[serde(rename = "PAYOUT")]
    Payout,
    #[serde(rename = "PIS_PAY_IN")]
    PisPayIn,
    #[serde(rename = "REFUND")]
    Refund,
    #[serde(rename = "FUNDS_SWEEP")]
    FundsSweep,
    #[serde(rename = "RETURN")]
    Return,
}

impl Default for Type {
    fn default() -> Type {
        Self::BankTransfer
    }
}

