/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferEventType : The type of event that this transfer represents. Event types with prefix `sweep` represents events for Plaid Ledger sweeps.  `pending`: A new transfer was created; it is in the pending state.  `cancelled`: The transfer was cancelled by the client.  `failed`: The transfer failed, no funds were moved.  `posted`: The transfer has been successfully submitted to the payment network.  `settled`: Credits are available to be withdrawn or debits have been deducted from the Plaid linked account.  `returned`: A posted transfer was returned.  `swept`: The transfer was swept to / from the sweep account.  `swept_settled`: Credits are available to be withdrawn or debits have been deducted from the customer’s business checking account.  `return_swept`: Due to the transfer being returned, funds were pulled from or pushed back to the sweep account.  `sweep.pending`: A new ledger sweep was created; it is in the pending state.  `sweep.posted`: The ledger sweep has been successfully submitted to the payment network.  `sweep.settled`: The transaction has settled in the funding account. This means that funds withdrawn from Plaid Ledger balance have reached the funding account, or funds to be deposited into the Plaid Ledger Balance have been pulled, and the hold period has begun.  `sweep.returned`: A posted ledger sweep was returned.  `sweep.failed`: The ledger sweep failed, no funds were moved.  `refund.pending`: A new refund was created; it is in the pending state.  `refund.cancelled`: The refund was cancelled.  `refund.failed`: The refund failed, no funds were moved.  `refund.posted`: The refund has been successfully submitted to the payment network.  `refund.settled`: The refund transaction has settled in the Plaid linked account.  `refund.returned`: A posted refund was returned.  `refund.swept`: The refund was swept from the sweep account.  `refund.return_swept`: Due to the refund being returned, funds were pushed back to the sweep account.

/// The type of event that this transfer represents. Event types with prefix `sweep` represents events for Plaid Ledger sweeps.  `pending`: A new transfer was created; it is in the pending state.  `cancelled`: The transfer was cancelled by the client.  `failed`: The transfer failed, no funds were moved.  `posted`: The transfer has been successfully submitted to the payment network.  `settled`: Credits are available to be withdrawn or debits have been deducted from the Plaid linked account.  `returned`: A posted transfer was returned.  `swept`: The transfer was swept to / from the sweep account.  `swept_settled`: Credits are available to be withdrawn or debits have been deducted from the customer’s business checking account.  `return_swept`: Due to the transfer being returned, funds were pulled from or pushed back to the sweep account.  `sweep.pending`: A new ledger sweep was created; it is in the pending state.  `sweep.posted`: The ledger sweep has been successfully submitted to the payment network.  `sweep.settled`: The transaction has settled in the funding account. This means that funds withdrawn from Plaid Ledger balance have reached the funding account, or funds to be deposited into the Plaid Ledger Balance have been pulled, and the hold period has begun.  `sweep.returned`: A posted ledger sweep was returned.  `sweep.failed`: The ledger sweep failed, no funds were moved.  `refund.pending`: A new refund was created; it is in the pending state.  `refund.cancelled`: The refund was cancelled.  `refund.failed`: The refund failed, no funds were moved.  `refund.posted`: The refund has been successfully submitted to the payment network.  `refund.settled`: The refund transaction has settled in the Plaid linked account.  `refund.returned`: A posted refund was returned.  `refund.swept`: The refund was swept from the sweep account.  `refund.return_swept`: Due to the refund being returned, funds were pushed back to the sweep account.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferEventType {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "posted")]
    Posted,
    #[serde(rename = "settled")]
    Settled,
    #[serde(rename = "returned")]
    Returned,
    #[serde(rename = "swept")]
    Swept,
    #[serde(rename = "swept_settled")]
    SweptSettled,
    #[serde(rename = "return_swept")]
    ReturnSwept,
    #[serde(rename = "sweep.pending")]
    SweepPeriodPending,
    #[serde(rename = "sweep.posted")]
    SweepPeriodPosted,
    #[serde(rename = "sweep.settled")]
    SweepPeriodSettled,
    #[serde(rename = "sweep.returned")]
    SweepPeriodReturned,
    #[serde(rename = "sweep.failed")]
    SweepPeriodFailed,

}

impl ToString for TransferEventType {
    fn to_string(&self) -> String {
        match self {
            Self::Pending => String::from("pending"),
            Self::Cancelled => String::from("cancelled"),
            Self::Failed => String::from("failed"),
            Self::Posted => String::from("posted"),
            Self::Settled => String::from("settled"),
            Self::Returned => String::from("returned"),
            Self::Swept => String::from("swept"),
            Self::SweptSettled => String::from("swept_settled"),
            Self::ReturnSwept => String::from("return_swept"),
            Self::SweepPeriodPending => String::from("sweep.pending"),
            Self::SweepPeriodPosted => String::from("sweep.posted"),
            Self::SweepPeriodSettled => String::from("sweep.settled"),
            Self::SweepPeriodReturned => String::from("sweep.returned"),
            Self::SweepPeriodFailed => String::from("sweep.failed"),
        }
    }
}

impl Default for TransferEventType {
    fn default() -> TransferEventType {
        Self::Pending
    }
}




