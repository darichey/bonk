/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferRepaymentReturn : Represents a return on a Guaranteed ACH transfer that is included in the specified repayment.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferRepaymentReturn {
    /// The unique identifier of the guaranteed transfer that was returned.
    #[serde(rename = "transfer_id")]
    pub transfer_id: String,
    /// The unique identifier of the corresponding `returned` transfer event.
    #[serde(rename = "event_id")]
    pub event_id: i32,
    /// The value of the returned transfer.
    #[serde(rename = "amount")]
    pub amount: String,
    /// The currency of the repayment, e.g. \"USD\".
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
}

impl TransferRepaymentReturn {
    /// Represents a return on a Guaranteed ACH transfer that is included in the specified repayment.
    pub fn new(transfer_id: String, event_id: i32, amount: String, iso_currency_code: String) -> TransferRepaymentReturn {
        TransferRepaymentReturn {
            transfer_id,
            event_id,
            amount,
            iso_currency_code,
        }
    }
}


