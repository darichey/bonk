/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferRepayment : A repayment is created automatically after one or more guaranteed transactions receive a return. If there are multiple eligible returns in a day, they are batched together into a single repayment.  Repayments are sent over ACH, with funds typically available on the next banking day.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferRepayment {
    /// Identifier of the repayment.
    #[serde(rename = "repayment_id")]
    pub repayment_id: String,
    /// The datetime when the repayment occurred, in RFC 3339 format.
    #[serde(rename = "created")]
    pub created: String,
    /// Decimal amount of the repayment as it appears on your account ledger.
    #[serde(rename = "amount")]
    pub amount: String,
    /// The currency of the repayment, e.g. \"USD\".
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
}

impl TransferRepayment {
    /// A repayment is created automatically after one or more guaranteed transactions receive a return. If there are multiple eligible returns in a day, they are batched together into a single repayment.  Repayments are sent over ACH, with funds typically available on the next banking day.
    pub fn new(repayment_id: String, created: String, amount: String, iso_currency_code: String) -> TransferRepayment {
        TransferRepayment {
            repayment_id,
            created,
            amount,
            iso_currency_code,
        }
    }
}


