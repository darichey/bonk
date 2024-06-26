/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionOverride : Data to populate as test transaction data. If not specified, random transactions will be generated instead.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionOverride {
    /// The date of the transaction, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format. Transactions in Sandbox will move from pending to posted once their transaction date has been reached. If a `date_transacted` is not provided by the institution, a transaction date may be available in the [`authorized_date`](https://plaid.com/docs/api/products/transactions/#transactions-get-response-transactions-authorized-date) field.
    #[serde(rename = "date_transacted")]
    pub date_transacted: String,
    /// The date the transaction posted, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format. Posted dates in the past or present will result in posted transactions; posted dates in the future will result in pending transactions.
    #[serde(rename = "date_posted")]
    pub date_posted: String,
    /// The transaction amount. Can be negative.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// The transaction description.
    #[serde(rename = "description")]
    pub description: String,
    /// The ISO-4217 format currency code for the transaction.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl TransactionOverride {
    /// Data to populate as test transaction data. If not specified, random transactions will be generated instead.
    pub fn new(date_transacted: String, date_posted: String, amount: f64, description: String) -> TransactionOverride {
        TransactionOverride {
            date_transacted,
            date_posted,
            amount,
            description,
            currency: None,
        }
    }
}


