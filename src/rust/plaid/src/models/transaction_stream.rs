/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionStream : A grouping of related transactions



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionStream {
    /// The ID of the account to which the stream belongs
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// A unique id for the stream
    #[serde(rename = "stream_id")]
    pub stream_id: String,
    /// A hierarchical array of the categories to which this transaction belongs. See [Categories](https://plaid.com/docs/api/products/transactions/#categoriesget).  All implementations are encouraged to use the new `personal_finance_category` instead of `category`. `personal_finance_category` provides more meaningful categorization and greater accuracy.
    #[serde(rename = "category")]
    pub category: Vec<String>,
    /// The ID of the category to which this transaction belongs. See [Categories](https://plaid.com/docs/api/products/transactions/#categoriesget).  All implementations are encouraged to use the new `personal_finance_category` instead of `category`. `personal_finance_category` provides more meaningful categorization and greater accuracy.
    #[serde(rename = "category_id")]
    pub category_id: String,
    /// A description of the transaction stream.
    #[serde(rename = "description")]
    pub description: String,
    /// The merchant associated with the transaction stream.
    #[serde(rename = "merchant_name", deserialize_with = "Option::deserialize")]
    pub merchant_name: Option<String>,
    /// The posted date of the earliest transaction in the stream.
    #[serde(rename = "first_date")]
    pub first_date: String,
    /// The posted date of the latest transaction in the stream.
    #[serde(rename = "last_date")]
    pub last_date: String,
    /// The predicted date of the next payment. This will only be set if the next payment date can be predicted.
    #[serde(rename = "predicted_next_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub predicted_next_date: Option<Option<String>>,
    #[serde(rename = "frequency")]
    pub frequency: crate::models::RecurringTransactionFrequency,
    /// An array of Plaid transaction IDs belonging to the stream, sorted by posted date.
    #[serde(rename = "transaction_ids")]
    pub transaction_ids: Vec<String>,
    #[serde(rename = "average_amount")]
    pub average_amount: crate::models::TransactionStreamAmount,
    #[serde(rename = "last_amount")]
    pub last_amount: crate::models::TransactionStreamAmount,
    /// Indicates whether the transaction stream is still live.
    #[serde(rename = "is_active")]
    pub is_active: bool,
    #[serde(rename = "status")]
    pub status: crate::models::TransactionStreamStatus,
    #[serde(rename = "personal_finance_category", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub personal_finance_category: Option<Option<crate::models::PersonalFinanceCategory>>,
    /// This will be set to `true` if the stream has been modified by request to a `/transactions/recurring/streams` endpoint. It will be `false` for all other streams.
    #[serde(rename = "is_user_modified")]
    pub is_user_modified: bool,
    /// The date and time of the most recent user modification. This will only be set if `is_user_modified` is `true`.
    #[serde(rename = "last_user_modified_datetime", skip_serializing_if = "Option::is_none")]
    pub last_user_modified_datetime: Option<String>,
}

impl TransactionStream {
    /// A grouping of related transactions
    pub fn new(account_id: String, stream_id: String, category: Vec<String>, category_id: String, description: String, merchant_name: Option<String>, first_date: String, last_date: String, frequency: crate::models::RecurringTransactionFrequency, transaction_ids: Vec<String>, average_amount: crate::models::TransactionStreamAmount, last_amount: crate::models::TransactionStreamAmount, is_active: bool, status: crate::models::TransactionStreamStatus, is_user_modified: bool) -> TransactionStream {
        TransactionStream {
            account_id,
            stream_id,
            category,
            category_id,
            description,
            merchant_name,
            first_date,
            last_date,
            predicted_next_date: None,
            frequency,
            transaction_ids,
            average_amount,
            last_amount,
            is_active,
            status,
            personal_finance_category: None,
            is_user_modified,
            last_user_modified_datetime: None,
        }
    }
}


