/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InvestmentTransaction : A transaction within an investment account.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvestmentTransaction {
    /// The ID of the Investment transaction, unique across all Plaid transactions. Like all Plaid identifiers, the `investment_transaction_id` is case sensitive.
    #[serde(rename = "investment_transaction_id")]
    pub investment_transaction_id: String,
    /// A legacy field formerly used internally by Plaid to identify certain canceled transactions.
    #[serde(rename = "cancel_transaction_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cancel_transaction_id: Option<Option<String>>,
    /// The `account_id` of the account against which this transaction posted.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The `security_id` to which this transaction is related.
    #[serde(rename = "security_id", deserialize_with = "Option::deserialize")]
    pub security_id: Option<String>,
    /// The [ISO 8601](https://wikipedia.org/wiki/ISO_8601) posting date for the transaction.
    #[serde(rename = "date")]
    pub date: String,
    /// The institution’s description of the transaction.
    #[serde(rename = "name")]
    pub name: String,
    /// The number of units of the security involved in this transaction. Positive for buy transactions; negative for sell transactions.
    #[serde(rename = "quantity")]
    pub quantity: f64,
    /// The complete value of the transaction. Positive values when cash is debited, e.g. purchases of stock; negative values when cash is credited, e.g. sales of stock. Treatment remains the same for cash-only movements unassociated with securities.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// The price of the security at which this transaction occurred.
    #[serde(rename = "price")]
    pub price: f64,
    /// The combined value of all fees applied to this transaction
    #[serde(rename = "fees", deserialize_with = "Option::deserialize")]
    pub fees: Option<f64>,
    #[serde(rename = "type")]
    pub r#type: crate::models::InvestmentTransactionType,
    #[serde(rename = "subtype")]
    pub subtype: crate::models::InvestmentTransactionSubtype,
    /// The ISO-4217 currency code of the transaction. Always `null` if `unofficial_currency_code` is non-`null`.
    #[serde(rename = "iso_currency_code", deserialize_with = "Option::deserialize")]
    pub iso_currency_code: Option<String>,
    /// The unofficial currency code associated with the holding. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.
    #[serde(rename = "unofficial_currency_code", deserialize_with = "Option::deserialize")]
    pub unofficial_currency_code: Option<String>,
}

impl InvestmentTransaction {
    /// A transaction within an investment account.
    pub fn new(investment_transaction_id: String, account_id: String, security_id: Option<String>, date: String, name: String, quantity: f64, amount: f64, price: f64, fees: Option<f64>, r#type: crate::models::InvestmentTransactionType, subtype: crate::models::InvestmentTransactionSubtype, iso_currency_code: Option<String>, unofficial_currency_code: Option<String>) -> InvestmentTransaction {
        InvestmentTransaction {
            investment_transaction_id,
            cancel_transaction_id: None,
            account_id,
            security_id,
            date,
            name,
            quantity,
            amount,
            price,
            fees,
            r#type,
            subtype,
            iso_currency_code,
            unofficial_currency_code,
        }
    }
}


