/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationPaymentStatus : The status of the payment.  `PAYMENT_STATUS_INPUT_NEEDED`: This is the initial state of all payments. It indicates that the payment is waiting on user input to continue processing. A payment may re-enter this state later on if further input is needed.  `PAYMENT_STATUS_INITIATED`: The payment has been successfully authorised and accepted by the financial institution. For successful payments, this is a potential terminal status. Further status transitions can be to REJECTED and, when supported by the institution, to EXECUTED.  `PAYMENT_STATUS_INSUFFICIENT_FUNDS`: The payment has failed due to insufficient funds.  `PAYMENT_STATUS_FAILED`: The payment has failed to be initiated. This error may be caused by transient system outages and is retryable once the root cause is resolved.  `PAYMENT_STATUS_BLOCKED`: The payment has been blocked by Plaid. This can occur, for example, due to Plaid flagging the payment as potentially risky. This is a retryable error.  `PAYMENT_STATUS_AUTHORISING`: The payment is currently being processed. The payment will automatically exit this state when the financial institution has authorised the transaction.  `PAYMENT_STATUS_CANCELLED`: The payment was cancelled (typically by the end user) during authorisation.  `PAYMENT_STATUS_EXECUTED`: The funds have successfully left the payer account and payment is considered complete. Not all institutions support this status: support is more common in the UK, and less common in the EU. For institutions where this status is not supported, the terminal status for a successful payment will be `PAYMENT_STATUS_INITIATED`.  `PAYMENT_STATUS_SETTLED`: The payment has settled and funds are available for use. A payment will typically settle within seconds to several days, depending on which payment rail is used. This status is only available to customers using [Plaid Virtual Accounts](https://plaid.com/docs/virtual-accounts/).  `PAYMENT_STATUS_ESTABLISHED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.  `PAYMENT_STATUS_REJECTED`: The payment was rejected by the financial institution.  Deprecated: These statuses will be removed in a future release.  `PAYMENT_STATUS_UNKNOWN`: The payment status is unknown.  `PAYMENT_STATUS_PROCESSING`: The payment is currently being processed. The payment will automatically exit this state when processing is complete.  `PAYMENT_STATUS_COMPLETED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.

/// The status of the payment.  `PAYMENT_STATUS_INPUT_NEEDED`: This is the initial state of all payments. It indicates that the payment is waiting on user input to continue processing. A payment may re-enter this state later on if further input is needed.  `PAYMENT_STATUS_INITIATED`: The payment has been successfully authorised and accepted by the financial institution. For successful payments, this is a potential terminal status. Further status transitions can be to REJECTED and, when supported by the institution, to EXECUTED.  `PAYMENT_STATUS_INSUFFICIENT_FUNDS`: The payment has failed due to insufficient funds.  `PAYMENT_STATUS_FAILED`: The payment has failed to be initiated. This error may be caused by transient system outages and is retryable once the root cause is resolved.  `PAYMENT_STATUS_BLOCKED`: The payment has been blocked by Plaid. This can occur, for example, due to Plaid flagging the payment as potentially risky. This is a retryable error.  `PAYMENT_STATUS_AUTHORISING`: The payment is currently being processed. The payment will automatically exit this state when the financial institution has authorised the transaction.  `PAYMENT_STATUS_CANCELLED`: The payment was cancelled (typically by the end user) during authorisation.  `PAYMENT_STATUS_EXECUTED`: The funds have successfully left the payer account and payment is considered complete. Not all institutions support this status: support is more common in the UK, and less common in the EU. For institutions where this status is not supported, the terminal status for a successful payment will be `PAYMENT_STATUS_INITIATED`.  `PAYMENT_STATUS_SETTLED`: The payment has settled and funds are available for use. A payment will typically settle within seconds to several days, depending on which payment rail is used. This status is only available to customers using [Plaid Virtual Accounts](https://plaid.com/docs/virtual-accounts/).  `PAYMENT_STATUS_ESTABLISHED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.  `PAYMENT_STATUS_REJECTED`: The payment was rejected by the financial institution.  Deprecated: These statuses will be removed in a future release.  `PAYMENT_STATUS_UNKNOWN`: The payment status is unknown.  `PAYMENT_STATUS_PROCESSING`: The payment is currently being processed. The payment will automatically exit this state when processing is complete.  `PAYMENT_STATUS_COMPLETED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentInitiationPaymentStatus {
    #[serde(rename = "PAYMENT_STATUS_INPUT_NEEDED")]
    InputNeeded,
    #[serde(rename = "PAYMENT_STATUS_PROCESSING")]
    Processing,
    #[serde(rename = "PAYMENT_STATUS_INITIATED")]
    Initiated,
    #[serde(rename = "PAYMENT_STATUS_COMPLETED")]
    Completed,
    #[serde(rename = "PAYMENT_STATUS_INSUFFICIENT_FUNDS")]
    InsufficientFunds,
    #[serde(rename = "PAYMENT_STATUS_FAILED")]
    Failed,
    #[serde(rename = "PAYMENT_STATUS_BLOCKED")]
    Blocked,
    #[serde(rename = "PAYMENT_STATUS_UNKNOWN")]
    Unknown,
    #[serde(rename = "PAYMENT_STATUS_EXECUTED")]
    Executed,
    #[serde(rename = "PAYMENT_STATUS_SETTLED")]
    Settled,
    #[serde(rename = "PAYMENT_STATUS_AUTHORISING")]
    Authorising,
    #[serde(rename = "PAYMENT_STATUS_CANCELLED")]
    Cancelled,
    #[serde(rename = "PAYMENT_STATUS_ESTABLISHED")]
    Established,
    #[serde(rename = "PAYMENT_STATUS_REJECTED")]
    Rejected,

}

impl ToString for PaymentInitiationPaymentStatus {
    fn to_string(&self) -> String {
        match self {
            Self::InputNeeded => String::from("PAYMENT_STATUS_INPUT_NEEDED"),
            Self::Processing => String::from("PAYMENT_STATUS_PROCESSING"),
            Self::Initiated => String::from("PAYMENT_STATUS_INITIATED"),
            Self::Completed => String::from("PAYMENT_STATUS_COMPLETED"),
            Self::InsufficientFunds => String::from("PAYMENT_STATUS_INSUFFICIENT_FUNDS"),
            Self::Failed => String::from("PAYMENT_STATUS_FAILED"),
            Self::Blocked => String::from("PAYMENT_STATUS_BLOCKED"),
            Self::Unknown => String::from("PAYMENT_STATUS_UNKNOWN"),
            Self::Executed => String::from("PAYMENT_STATUS_EXECUTED"),
            Self::Settled => String::from("PAYMENT_STATUS_SETTLED"),
            Self::Authorising => String::from("PAYMENT_STATUS_AUTHORISING"),
            Self::Cancelled => String::from("PAYMENT_STATUS_CANCELLED"),
            Self::Established => String::from("PAYMENT_STATUS_ESTABLISHED"),
            Self::Rejected => String::from("PAYMENT_STATUS_REJECTED"),
        }
    }
}

impl Default for PaymentInitiationPaymentStatus {
    fn default() -> PaymentInitiationPaymentStatus {
        Self::InputNeeded
    }
}




