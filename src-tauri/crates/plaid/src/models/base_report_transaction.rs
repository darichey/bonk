/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BaseReportTransaction : A transaction on the Base Report



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BaseReportTransaction {
    /// The settled value of the transaction, denominated in the transaction's currency, as stated in `iso_currency_code` or `unofficial_currency_code`. Positive values when money moves out of the account; negative values when money moves in. For example, debit card purchases are positive; credit card payments, direct deposits, and refunds are negative.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// The ISO-4217 currency code of the transaction. Always `null` if `unofficial_currency_code` is non-null.
    #[serde(rename = "iso_currency_code", deserialize_with = "Option::deserialize")]
    pub iso_currency_code: Option<String>,
    /// The unofficial currency code associated with the transaction. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `unofficial_currency_code`s.
    #[serde(rename = "unofficial_currency_code", deserialize_with = "Option::deserialize")]
    pub unofficial_currency_code: Option<String>,
    /// The string returned by the financial institution to describe the transaction.
    #[serde(rename = "original_description", deserialize_with = "Option::deserialize")]
    pub original_description: Option<String>,
    #[serde(rename = "credit_category", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub credit_category: Option<Option<crate::models::CreditCategory>>,
    /// The check number of the transaction. This field is only populated for check transactions.
    #[serde(rename = "check_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub check_number: Option<Option<String>>,
    /// For pending transactions, the date that the transaction occurred; for posted transactions, the date that the transaction posted. Both dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DD` ).
    #[serde(rename = "date")]
    pub date: String,
    /// The date on which the transaction took place, in IS0 8601 format.
    #[serde(rename = "date_transacted", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_transacted: Option<Option<String>>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<crate::models::Location>,
    /// The merchant name, as enriched by Plaid from the `name` field. This is typically a more human-readable version of the merchant counterparty in the transaction. For some bank transactions (such as checks or account transfers) where there is no meaningful merchant name, this value will be `null`.
    #[serde(rename = "merchant_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<Option<String>>,
    /// When `true`, identifies the transaction as pending or unsettled. Pending transaction details (name, type, amount, category ID) may change before they are settled.
    #[serde(rename = "pending")]
    pub pending: bool,
    /// The name of the account owner. This field is not typically populated and only relevant when dealing with sub-accounts.
    #[serde(rename = "account_owner", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_owner: Option<Option<String>>,
}

impl BaseReportTransaction {
    /// A transaction on the Base Report
    pub fn new(amount: f64, iso_currency_code: Option<String>, unofficial_currency_code: Option<String>, original_description: Option<String>, date: String, pending: bool) -> BaseReportTransaction {
        BaseReportTransaction {
            amount,
            iso_currency_code,
            unofficial_currency_code,
            original_description,
            credit_category: None,
            check_number: None,
            date,
            date_transacted: None,
            location: None,
            merchant_name: None,
            pending,
            account_owner: None,
        }
    }
}


