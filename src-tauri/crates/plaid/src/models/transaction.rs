/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Transaction : A representation of a transaction



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Transaction {
    /// The ID of the account in which this transaction occurred.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The settled value of the transaction, denominated in the transactions's currency, as stated in `iso_currency_code` or `unofficial_currency_code`. Positive values when money moves out of the account; negative values when money moves in. For example, debit card purchases are positive; credit card payments, direct deposits, and refunds are negative.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// The ISO-4217 currency code of the transaction. Always `null` if `unofficial_currency_code` is non-null.
    #[serde(rename = "iso_currency_code", deserialize_with = "Option::deserialize")]
    pub iso_currency_code: Option<String>,
    /// The unofficial currency code associated with the transaction. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.
    #[serde(rename = "unofficial_currency_code", deserialize_with = "Option::deserialize")]
    pub unofficial_currency_code: Option<String>,
    /// A hierarchical array of the categories to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).  All Transactions implementations are recommended to use the new `personal_finance_category` instead of `category`, as it provides greater accuracy and more meaningful categorization.  If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.
    #[serde(rename = "category", deserialize_with = "Option::deserialize")]
    pub category: Option<Vec<String>>,
    /// The ID of the category to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).  All Transactions implementations are recommended to use the new `personal_finance_category` instead of `category`, as it provides greater accuracy and more meaningful categorization.  If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.
    #[serde(rename = "category_id", deserialize_with = "Option::deserialize")]
    pub category_id: Option<String>,
    /// The check number of the transaction. This field is only populated for check transactions.
    #[serde(rename = "check_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub check_number: Option<Option<String>>,
    /// For pending transactions, the date that the transaction occurred; for posted transactions, the date that the transaction posted. Both dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DD` ). To receive information about the date that a posted transaction was initiated, see the `authorized_date` field.
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "location")]
    pub location: crate::models::Location,
    /// The merchant name or transaction description.  If the `transactions` object was returned by a Transactions endpoint such as `/transactions/sync` or `/transactions/get`, this field will always appear. If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.
    #[serde(rename = "name")]
    pub name: String,
    /// The merchant name, as enriched by Plaid from the `name` field. This is typically a more human-readable version of the merchant counterparty in the transaction. For some bank transactions (such as checks or account transfers) where there is no meaningful merchant name, this value will be `null`.
    #[serde(rename = "merchant_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<Option<String>>,
    /// The string returned by the financial institution to describe the transaction. For transactions returned by `/transactions/sync` or `/transactions/get`, this field is in beta and will be omitted unless the client is both enrolled in the closed beta program and has set `options.include_original_description` to `true`.
    #[serde(rename = "original_description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub original_description: Option<Option<String>>,
    #[serde(rename = "payment_meta")]
    pub payment_meta: crate::models::PaymentMeta,
    /// When `true`, identifies the transaction as pending or unsettled. Pending transaction details (name, type, amount, category ID) may change before they are settled.
    #[serde(rename = "pending")]
    pub pending: bool,
    /// The ID of a posted transaction's associated pending transaction, where applicable.
    #[serde(rename = "pending_transaction_id", deserialize_with = "Option::deserialize")]
    pub pending_transaction_id: Option<String>,
    /// The name of the account owner. This field is not typically populated and only relevant when dealing with sub-accounts.
    #[serde(rename = "account_owner", deserialize_with = "Option::deserialize")]
    pub account_owner: Option<String>,
    /// The unique ID of the transaction. Like all Plaid identifiers, the `transaction_id` is case sensitive.
    #[serde(rename = "transaction_id")]
    pub transaction_id: String,
    /// Please use the `payment_channel` field, `transaction_type` will be deprecated in the future.  `digital:` transactions that took place online.  `place:` transactions that were made at a physical location.  `special:` transactions that relate to banks, e.g. fees or deposits.  `unresolved:` transactions that do not fit into the other three types. 
    #[serde(rename = "transaction_type", skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<TransactionType>,
    /// The URL of a logo associated with this transaction, if available. The logo will always be 100×100 pixel PNG file.
    #[serde(rename = "logo_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<Option<String>>,
    /// The website associated with this transaction, if available.
    #[serde(rename = "website", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub website: Option<Option<String>>,
    /// The date that the transaction was authorized. For posted transactions, the `date` field will indicate the posted date, but `authorized_date` will indicate the day the transaction was authorized by the financial institution. If presenting transactions to the user in a UI, the `authorized_date`, when available, is generally preferable to use over the `date` field for posted transactions, as it will generally represent the date the user actually made the transaction. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DD` ).
    #[serde(rename = "authorized_date", deserialize_with = "Option::deserialize")]
    pub authorized_date: Option<String>,
    /// Date and time when a transaction was authorized in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DDTHH:mm:ssZ` ). For posted transactions, the `datetime` field will indicate the posted date, but `authorized_datetime` will indicate the day the transaction was authorized by the financial institution. If presenting transactions to the user in a UI, the `authorized_datetime`, when available, is generally preferable to use over the `datetime` field for posted transactions, as it will generally represent the date the user actually made the transaction.  This field is returned for select financial institutions and comes as provided by the institution. It may contain default time values (such as 00:00:00). This field is only populated in API version 2019-05-29 and later.
    #[serde(rename = "authorized_datetime", deserialize_with = "Option::deserialize")]
    pub authorized_datetime: Option<String>,
    /// Date and time when a transaction was posted in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DDTHH:mm:ssZ` ). For the date that the transaction was initiated, rather than posted, see the `authorized_datetime` field.  This field is returned for select financial institutions and comes as provided by the institution. It may contain default time values (such as 00:00:00). This field is only populated in API version 2019-05-29 and later.
    #[serde(rename = "datetime", deserialize_with = "Option::deserialize")]
    pub datetime: Option<String>,
    /// The channel used to make a payment. `online:` transactions that took place online.  `in store:` transactions that were made at a physical location.  `other:` transactions that relate to banks, e.g. fees or deposits.  This field replaces the `transaction_type` field. 
    #[serde(rename = "payment_channel")]
    pub payment_channel: PaymentChannel,
    #[serde(rename = "personal_finance_category", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub personal_finance_category: Option<Option<crate::models::PersonalFinanceCategory>>,
    #[serde(rename = "transaction_code", deserialize_with = "Option::deserialize")]
    pub transaction_code: Option<crate::models::TransactionCode>,
    /// The URL of an icon associated with the primary personal finance category. The icon will always be 100×100 pixel PNG file.
    #[serde(rename = "personal_finance_category_icon_url", skip_serializing_if = "Option::is_none")]
    pub personal_finance_category_icon_url: Option<String>,
    /// The counterparties present in the transaction. Counterparties, such as the merchant or the financial institution, are extracted by Plaid from the raw description.
    #[serde(rename = "counterparties", skip_serializing_if = "Option::is_none")]
    pub counterparties: Option<Vec<crate::models::TransactionCounterparty>>,
    /// A unique, stable, Plaid-generated ID that maps to the merchant.
    #[serde(rename = "merchant_entity_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub merchant_entity_id: Option<Option<String>>,
}

impl Transaction {
    /// A representation of a transaction
    pub fn new(account_id: String, amount: f64, iso_currency_code: Option<String>, unofficial_currency_code: Option<String>, category: Option<Vec<String>>, category_id: Option<String>, date: String, location: crate::models::Location, name: String, payment_meta: crate::models::PaymentMeta, pending: bool, pending_transaction_id: Option<String>, account_owner: Option<String>, transaction_id: String, authorized_date: Option<String>, authorized_datetime: Option<String>, datetime: Option<String>, payment_channel: PaymentChannel, transaction_code: Option<crate::models::TransactionCode>) -> Transaction {
        Transaction {
            account_id,
            amount,
            iso_currency_code,
            unofficial_currency_code,
            category,
            category_id,
            check_number: None,
            date,
            location,
            name,
            merchant_name: None,
            original_description: None,
            payment_meta,
            pending,
            pending_transaction_id,
            account_owner,
            transaction_id,
            transaction_type: None,
            logo_url: None,
            website: None,
            authorized_date,
            authorized_datetime,
            datetime,
            payment_channel,
            personal_finance_category: None,
            transaction_code,
            personal_finance_category_icon_url: None,
            counterparties: None,
            merchant_entity_id: None,
        }
    }
}

/// Please use the `payment_channel` field, `transaction_type` will be deprecated in the future.  `digital:` transactions that took place online.  `place:` transactions that were made at a physical location.  `special:` transactions that relate to banks, e.g. fees or deposits.  `unresolved:` transactions that do not fit into the other three types. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionType {
    #[serde(rename = "digital")]
    Digital,
    #[serde(rename = "place")]
    Place,
    #[serde(rename = "special")]
    Special,
    #[serde(rename = "unresolved")]
    Unresolved,
}

impl Default for TransactionType {
    fn default() -> TransactionType {
        Self::Digital
    }
}
/// The channel used to make a payment. `online:` transactions that took place online.  `in store:` transactions that were made at a physical location.  `other:` transactions that relate to banks, e.g. fees or deposits.  This field replaces the `transaction_type` field. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentChannel {
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "in store")]
    InStore,
    #[serde(rename = "other")]
    Other,
}

impl Default for PaymentChannel {
    fn default() -> PaymentChannel {
        Self::Online
    }
}

