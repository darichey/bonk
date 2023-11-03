/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraBankIncomeTransaction : The transactions data for the end user's income source(s).



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CraBankIncomeTransaction {
    /// The settled value of the transaction, denominated in the transactions's currency as stated in `iso_currency_code` or `unofficial_currency_code`. Positive values when money moves out of the account; negative values when money moves in. For example, credit card purchases are positive; credit card payment, direct deposits, and refunds are negative.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// For pending transactions, the date that the transaction occurred; for posted transactions, the date that the transaction posted. Both dates are returned in an ISO 8601 format (YYYY-MM-DD).
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// The merchant name or transaction description.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The string returned by the financial institution to describe the transaction.
    #[serde(rename = "original_description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub original_description: Option<Option<String>>,
    /// When true, identifies the transaction as pending or unsettled. Pending transaction details (name, type, amount, category ID) may change before they are settled.
    #[serde(rename = "pending", skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
    /// The check number of the transaction. This field is only populated for check transactions.
    #[serde(rename = "check_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub check_number: Option<Option<String>>,
    /// The ISO 4217 currency code of the amount or balance.
    #[serde(rename = "iso_currency_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<Option<String>>,
    /// The unofficial currency code associated with the amount or balance. Always `null` if `iso_currency_code` is non-null. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.
    #[serde(rename = "unofficial_currency_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<Option<String>>,
}

impl CraBankIncomeTransaction {
    /// The transactions data for the end user's income source(s).
    pub fn new() -> CraBankIncomeTransaction {
        CraBankIncomeTransaction {
            amount: None,
            date: None,
            name: None,
            original_description: None,
            pending: None,
            check_number: None,
            iso_currency_code: None,
            unofficial_currency_code: None,
        }
    }
}


