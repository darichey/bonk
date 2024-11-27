/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PrismCashScoreMetadata : An object containing metadata about the provided transactions.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PrismCashScoreMetadata {
    /// Number of days since the oldest transaction.
    #[serde(rename = "max_age", deserialize_with = "Option::deserialize")]
    pub max_age: Option<i32>,
    /// Number of days since the latest transaction.
    #[serde(rename = "min_age", deserialize_with = "Option::deserialize")]
    pub min_age: Option<i32>,
    /// Number of days since the latest credit transaction.
    #[serde(rename = "min_age_credit", deserialize_with = "Option::deserialize")]
    pub min_age_credit: Option<i32>,
    /// Number of days since the latest debit transaction.
    #[serde(rename = "min_age_debit", deserialize_with = "Option::deserialize")]
    pub min_age_debit: Option<i32>,
    /// Number of days since the oldest debit transaction.
    #[serde(rename = "max_age_debit", deserialize_with = "Option::deserialize")]
    pub max_age_debit: Option<i32>,
    /// Number of days since the oldest credit transaction.
    #[serde(rename = "max_age_credit", deserialize_with = "Option::deserialize")]
    pub max_age_credit: Option<i32>,
    /// Number of credit transactions.
    #[serde(rename = "num_trxn_credit", deserialize_with = "Option::deserialize")]
    pub num_trxn_credit: Option<i32>,
    /// Number of debit transactions.
    #[serde(rename = "num_trxn_debit", deserialize_with = "Option::deserialize")]
    pub num_trxn_debit: Option<i32>,
    /// Number of credit transactions in the last 30 days.
    #[serde(rename = "l1m_credit_value_cnt", deserialize_with = "Option::deserialize")]
    pub l1m_credit_value_cnt: Option<i32>,
    /// Number of debit transactions in the last 30 days.
    #[serde(rename = "l1m_debit_value_cnt", deserialize_with = "Option::deserialize")]
    pub l1m_debit_value_cnt: Option<i32>,
}

impl PrismCashScoreMetadata {
    /// An object containing metadata about the provided transactions.
    pub fn new(max_age: Option<i32>, min_age: Option<i32>, min_age_credit: Option<i32>, min_age_debit: Option<i32>, max_age_debit: Option<i32>, max_age_credit: Option<i32>, num_trxn_credit: Option<i32>, num_trxn_debit: Option<i32>, l1m_credit_value_cnt: Option<i32>, l1m_debit_value_cnt: Option<i32>) -> PrismCashScoreMetadata {
        PrismCashScoreMetadata {
            max_age,
            min_age,
            min_age_credit,
            min_age_debit,
            max_age_debit,
            max_age_credit,
            num_trxn_credit,
            num_trxn_debit,
            l1m_credit_value_cnt,
            l1m_debit_value_cnt,
        }
    }
}

