/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SignalEvaluateCoreAttributes : The core attributes object contains additional data that can be used to assess the ACH return risk. Examples of data include:  `days_since_first_plaid_connection`: The number of days since the first time the Item was connected to an application via Plaid `plaid_connections_count_7d`: The number of times the Item has been connected to applications via Plaid over the past 7 days `plaid_connections_count_30d`: The number of times the Item has been connected to applications via Plaid over the past 30 days `total_plaid_connections_count`: The number of times the Item has been connected to applications via Plaid `is_savings_or_money_market_account`: Indicates whether the ACH transaction funding account is a savings/money market account  For the full list and detailed documentation of core attributes available, or to request that core attributes not be returned, contact Sales or your Plaid account manager



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SignalEvaluateCoreAttributes {
    /// We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 7 days from the account that will be debited.
    #[serde(rename = "unauthorized_transactions_count_7d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unauthorized_transactions_count_7d: Option<Option<i32>>,
    /// We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 30 days from the account that will be debited.
    #[serde(rename = "unauthorized_transactions_count_30d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unauthorized_transactions_count_30d: Option<Option<i32>>,
    /// We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 60 days from the account that will be debited.
    #[serde(rename = "unauthorized_transactions_count_60d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unauthorized_transactions_count_60d: Option<Option<i32>>,
    /// We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 90 days from the account that will be debited.
    #[serde(rename = "unauthorized_transactions_count_90d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unauthorized_transactions_count_90d: Option<Option<i32>>,
    /// We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 7 days from the account that will be debited.
    #[serde(rename = "nsf_overdraft_transactions_count_7d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nsf_overdraft_transactions_count_7d: Option<Option<i32>>,
    /// We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 30 days from the account that will be debited.
    #[serde(rename = "nsf_overdraft_transactions_count_30d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nsf_overdraft_transactions_count_30d: Option<Option<i32>>,
    /// We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 60 days from the account that will be debited.
    #[serde(rename = "nsf_overdraft_transactions_count_60d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nsf_overdraft_transactions_count_60d: Option<Option<i32>>,
    /// We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 90 days from the account that will be debited.
    #[serde(rename = "nsf_overdraft_transactions_count_90d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nsf_overdraft_transactions_count_90d: Option<Option<i32>>,
    /// The number of days since the first time the Item was connected to an application via Plaid
    #[serde(rename = "days_since_first_plaid_connection", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub days_since_first_plaid_connection: Option<Option<i32>>,
    /// The number of times the Item has been connected to applications via Plaid over the past 7 days
    #[serde(rename = "plaid_connections_count_7d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub plaid_connections_count_7d: Option<Option<i32>>,
    /// The number of times the Item has been connected to applications via Plaid over the past 30 days
    #[serde(rename = "plaid_connections_count_30d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub plaid_connections_count_30d: Option<Option<i32>>,
    /// The total number of times the Item has been connected to applications via Plaid
    #[serde(rename = "total_plaid_connections_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_plaid_connections_count: Option<Option<i32>>,
    /// Indicates if the ACH transaction funding account is a savings/money market account
    #[serde(rename = "is_savings_or_money_market_account", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_savings_or_money_market_account: Option<Option<bool>>,
    /// The total credit (inflow) transaction amount over the past 10 days from the account that will be debited
    #[serde(rename = "total_credit_transactions_amount_10d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_credit_transactions_amount_10d: Option<Option<f64>>,
    /// The total debit (outflow) transaction amount over the past 10 days from the account that will be debited
    #[serde(rename = "total_debit_transactions_amount_10d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_debit_transactions_amount_10d: Option<Option<f64>>,
    /// The 50th percentile of all credit (inflow) transaction amounts over the past 28 days from the account that will be debited
    #[serde(rename = "p50_credit_transactions_amount_28d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub p50_credit_transactions_amount_28d: Option<Option<f64>>,
    /// The 50th percentile of all debit (outflow) transaction amounts over the past 28 days from the account that will be debited
    #[serde(rename = "p50_debit_transactions_amount_28d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub p50_debit_transactions_amount_28d: Option<Option<f64>>,
    /// The 95th percentile of all credit (inflow) transaction amounts over the past 28 days from the account that will be debited
    #[serde(rename = "p95_credit_transactions_amount_28d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub p95_credit_transactions_amount_28d: Option<Option<f64>>,
    /// The 95th percentile of all debit (outflow) transaction amounts over the past 28 days from the account that will be debited
    #[serde(rename = "p95_debit_transactions_amount_28d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub p95_debit_transactions_amount_28d: Option<Option<f64>>,
    /// The number of days within the past 90 days when the account that will be debited had a negative end-of-day available balance
    #[serde(rename = "days_with_negative_balance_count_90d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub days_with_negative_balance_count_90d: Option<Option<i32>>,
    /// The 90th percentile of the end-of-day available balance over the past 30 days of the account that will be debited
    #[serde(rename = "p90_eod_balance_30d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub p90_eod_balance_30d: Option<Option<f64>>,
    /// The 90th percentile of the end-of-day available balance over the past 60 days of the account that will be debited
    #[serde(rename = "p90_eod_balance_60d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub p90_eod_balance_60d: Option<Option<f64>>,
    /// The 90th percentile of the end-of-day available balance over the past 90 days of the account that will be debited
    #[serde(rename = "p90_eod_balance_90d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub p90_eod_balance_90d: Option<Option<f64>>,
    /// The 10th percentile of the end-of-day available balance over the past 30 days of the account that will be debited
    #[serde(rename = "p10_eod_balance_30d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub p10_eod_balance_30d: Option<Option<f64>>,
    /// The 10th percentile of the end-of-day available balance over the past 60 days of the account that will be debited
    #[serde(rename = "p10_eod_balance_60d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub p10_eod_balance_60d: Option<Option<f64>>,
    /// The 10th percentile of the end-of-day available balance over the past 90 days of the account that will be debited
    #[serde(rename = "p10_eod_balance_90d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub p10_eod_balance_90d: Option<Option<f64>>,
    /// Available balance, as of the `balance_last_updated` time. The available balance is the current balance less any outstanding holds or debits that have not yet posted to the account.
    #[serde(rename = "available_balance", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub available_balance: Option<Option<f64>>,
    /// Current balance, as of the `balance_last_updated` time. The current balance is the total amount of funds in the account.
    #[serde(rename = "current_balance", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub current_balance: Option<Option<f64>>,
    /// Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DDTHH:mm:ssZ) indicating the last time that the balance for the given account has been updated.
    #[serde(rename = "balance_last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub balance_last_updated: Option<Option<String>>,
    /// The number of times the account's phone numbers on file have changed over the past 28 days
    #[serde(rename = "phone_change_count_28d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone_change_count_28d: Option<Option<i32>>,
    /// The number of times the account's phone numbers on file have changed over the past 90 days
    #[serde(rename = "phone_change_count_90d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone_change_count_90d: Option<Option<i32>>,
    /// The number of times the account's email addresses on file have changed over the past 28 days
    #[serde(rename = "email_change_count_28d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub email_change_count_28d: Option<Option<i32>>,
    /// The number of times the account's email addresses on file have changed over the past 90 days
    #[serde(rename = "email_change_count_90d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub email_change_count_90d: Option<Option<i32>>,
    /// The number of times the account's addresses on file have changed over the past 28 days
    #[serde(rename = "address_change_count_28d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address_change_count_28d: Option<Option<i32>>,
    /// The number of times the account's addresses on file have changed over the past 90 days
    #[serde(rename = "address_change_count_90d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address_change_count_90d: Option<Option<i32>>,
    /// The number of non-OAuth authentication attempts via Plaid for this bank account over the past 3 days
    #[serde(rename = "plaid_non_oauth_authentication_attempts_count_3d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub plaid_non_oauth_authentication_attempts_count_3d: Option<Option<i32>>,
    /// The number of non-OAuth authentication attempts via Plaid for this bank account over the past 7 days
    #[serde(rename = "plaid_non_oauth_authentication_attempts_count_7d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub plaid_non_oauth_authentication_attempts_count_7d: Option<Option<i32>>,
    /// The number of non-OAuth authentication attempts via Plaid for this bank account over the past 30 days
    #[serde(rename = "plaid_non_oauth_authentication_attempts_count_30d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub plaid_non_oauth_authentication_attempts_count_30d: Option<Option<i32>>,
    /// The number of failed non-OAuth authentication attempts via Plaid for this bank account over the past 3 days
    #[serde(rename = "failed_plaid_non_oauth_authentication_attempts_count_3d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub failed_plaid_non_oauth_authentication_attempts_count_3d: Option<Option<i32>>,
    /// The number of failed non-OAuth authentication attempts via Plaid for this bank account over the past 7 days
    #[serde(rename = "failed_plaid_non_oauth_authentication_attempts_count_7d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub failed_plaid_non_oauth_authentication_attempts_count_7d: Option<Option<i32>>,
    /// The number of failed non-OAuth authentication attempts via Plaid for this bank account over the past 30 days
    #[serde(rename = "failed_plaid_non_oauth_authentication_attempts_count_30d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub failed_plaid_non_oauth_authentication_attempts_count_30d: Option<Option<i32>>,
    /// The total number of debit (outflow) transactions over the past 10 days from the account that will be debited
    #[serde(rename = "debit_transactions_count_10d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub debit_transactions_count_10d: Option<Option<i32>>,
    /// The total number of credit (inflow) transactions over the past 10 days from the account that will be debited
    #[serde(rename = "credit_transactions_count_10d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub credit_transactions_count_10d: Option<Option<i32>>,
    /// The total number of debit (outflow) transactions over the past 30 days from the account that will be debited
    #[serde(rename = "debit_transactions_count_30d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub debit_transactions_count_30d: Option<Option<i32>>,
    /// The total number of credit (inflow) transactions over the past 30 days from the account that will be debited
    #[serde(rename = "credit_transactions_count_30d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub credit_transactions_count_30d: Option<Option<i32>>,
    /// The total number of debit (outflow) transactions over the past 60 days from the account that will be debited
    #[serde(rename = "debit_transactions_count_60d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub debit_transactions_count_60d: Option<Option<i32>>,
    /// The total number of credit (inflow) transactions over the past 60 days from the account that will be debited
    #[serde(rename = "credit_transactions_count_60d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub credit_transactions_count_60d: Option<Option<i32>>,
    /// The total number of debit (outflow) transactions over the past 90 days from the account that will be debited
    #[serde(rename = "debit_transactions_count_90d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub debit_transactions_count_90d: Option<Option<i32>>,
    /// The total number of credit (inflow) transactions over the past 90 days from the account that will be debited
    #[serde(rename = "credit_transactions_count_90d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub credit_transactions_count_90d: Option<Option<i32>>,
    /// The total debit (outflow) transaction amount over the past 30 days from the account that will be debited
    #[serde(rename = "total_debit_transactions_amount_30d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_debit_transactions_amount_30d: Option<Option<f64>>,
    /// The total credit (inflow) transaction amount over the past 30 days from the account that will be debited
    #[serde(rename = "total_credit_transactions_amount_30d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_credit_transactions_amount_30d: Option<Option<f64>>,
    /// The total debit (outflow) transaction amount over the past 60 days from the account that will be debited
    #[serde(rename = "total_debit_transactions_amount_60d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_debit_transactions_amount_60d: Option<Option<f64>>,
    /// The total credit (inflow) transaction amount over the past 60 days from the account that will be debited
    #[serde(rename = "total_credit_transactions_amount_60d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_credit_transactions_amount_60d: Option<Option<f64>>,
    /// The total debit (outflow) transaction amount over the past 90 days from the account that will be debited
    #[serde(rename = "total_debit_transactions_amount_90d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_debit_transactions_amount_90d: Option<Option<f64>>,
    /// The total credit (inflow) transaction amount over the past 90 days from the account that will be debited
    #[serde(rename = "total_credit_transactions_amount_90d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_credit_transactions_amount_90d: Option<Option<f64>>,
    /// The 50th percentile of the end-of-day available balance over the past 30 days of the account that will be debited
    #[serde(rename = "p50_eod_balance_30d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub p50_eod_balance_30d: Option<Option<f64>>,
    /// The 50th percentile of the end-of-day available balance over the past 60 days of the account that will be debited
    #[serde(rename = "p50_eod_balance_60d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub p50_eod_balance_60d: Option<Option<f64>>,
    /// The 50th percentile of the end-of-day available balance over the past 90 days of the account that will be debited
    #[serde(rename = "p50_eod_balance_90d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub p50_eod_balance_90d: Option<Option<f64>>,
    /// The 50th percentile of the end-of-day available balance between day 31 and day 60 over the past 60 days of the account that will be debited
    #[serde(rename = "p50_eod_balance_31d_to_60d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub p50_eod_balance_31d_to_60d: Option<Option<f64>>,
    /// The 50th percentile of the end-of-day available balance between day 61 and day 90 over the past 60 days of the account that will be debited
    #[serde(rename = "p50_eod_balance_61d_to_90d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub p50_eod_balance_61d_to_90d: Option<Option<f64>>,
    /// The 90th percentile of the end-of-day available balance between day 31 and day 60 over the past 60 days of the account that will be debited
    #[serde(rename = "p90_eod_balance_31d_to_60d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub p90_eod_balance_31d_to_60d: Option<Option<f64>>,
    /// The 90th percentile of the end-of-day available balance between day 61 and day 90 over the past 60 days of the account that will be debited
    #[serde(rename = "p90_eod_balance_61d_to_90d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub p90_eod_balance_61d_to_90d: Option<Option<f64>>,
    /// The 10th percentile of the end-of-day available balance between day 31 and day 60 over the past 60 days of the account that will be debited
    #[serde(rename = "p10_eod_balance_31d_to_60d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub p10_eod_balance_31d_to_60d: Option<Option<f64>>,
    /// The 10th percentile of the end-of-day available balance between day 61 and day 90 over the past 60 days of the account that will be debited
    #[serde(rename = "p10_eod_balance_61d_to_90d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub p10_eod_balance_61d_to_90d: Option<Option<f64>>,
    /// Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DDTHH:mm:ssZ) indicating the last time that the transactions for the given account have been updated.
    #[serde(rename = "transactions_last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub transactions_last_updated: Option<Option<String>>,
    /// Indicates if the receiver bank account is closed
    #[serde(rename = "is_account_closed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_account_closed: Option<Option<bool>>,
    /// Indicates if the receiver bank account is either frozen or restricted
    #[serde(rename = "is_account_frozen_or_restricted", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_account_frozen_or_restricted: Option<Option<bool>>,
    /// The number of distinct IP addresses linked to the same bank account during Plaid authentication in the last 3 days
    #[serde(rename = "distinct_ip_addresses_count_3d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub distinct_ip_addresses_count_3d: Option<Option<i32>>,
    /// The number of distinct IP addresses linked to the same bank account during Plaid authentication in the last 7 days
    #[serde(rename = "distinct_ip_addresses_count_7d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub distinct_ip_addresses_count_7d: Option<Option<i32>>,
    /// The number of distinct IP addresses linked to the same bank account during Plaid authentication in the last 30 days (max 100)
    #[serde(rename = "distinct_ip_addresses_count_30d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub distinct_ip_addresses_count_30d: Option<Option<i32>>,
    /// The number of distinct IP addresses linked to the same bank account during Plaid authentication in the last 90 days (max 100)
    #[serde(rename = "distinct_ip_addresses_count_90d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub distinct_ip_addresses_count_90d: Option<Option<i32>>,
    /// The number of distinct user agents linked to the same bank account during Plaid authentication in the last 3 days
    #[serde(rename = "distinct_user_agents_count_3d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub distinct_user_agents_count_3d: Option<Option<i32>>,
    /// The number of distinct user agents linked to the same bank account during Plaid authentication in the last 7 days
    #[serde(rename = "distinct_user_agents_count_7d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub distinct_user_agents_count_7d: Option<Option<i32>>,
    /// The number of distinct user agents linked to the same bank account during Plaid authentication in the last 30 days
    #[serde(rename = "distinct_user_agents_count_30d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub distinct_user_agents_count_30d: Option<Option<i32>>,
    /// The number of distinct user agents linked to the same bank account during Plaid authentication in the last 90 days
    #[serde(rename = "distinct_user_agents_count_90d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub distinct_user_agents_count_90d: Option<Option<i32>>,
    /// The number of distinct SSL/TLS connection sessions linked to the same bank account during Plaid authentication in the last 3 days
    #[serde(rename = "distinct_ssl_tls_connection_sessions_count_3d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub distinct_ssl_tls_connection_sessions_count_3d: Option<Option<i32>>,
    /// The number of distinct SSL/TLS connection sessions linked to the same bank account during Plaid authentication in the last 7 days
    #[serde(rename = "distinct_ssl_tls_connection_sessions_count_7d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub distinct_ssl_tls_connection_sessions_count_7d: Option<Option<i32>>,
    /// The number of distinct SSL/TLS connection sessions linked to the same bank account during Plaid authentication in the last 30 days
    #[serde(rename = "distinct_ssl_tls_connection_sessions_count_30d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub distinct_ssl_tls_connection_sessions_count_30d: Option<Option<i32>>,
    /// The number of distinct SSL/TLS connection sessions linked to the same bank account during Plaid authentication in the last 90 days
    #[serde(rename = "distinct_ssl_tls_connection_sessions_count_90d", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub distinct_ssl_tls_connection_sessions_count_90d: Option<Option<i32>>,
    /// The number of days since the bank account was opened, as reported by the financial institution
    #[serde(rename = "days_since_account_opening", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub days_since_account_opening: Option<Option<i32>>,
}

impl SignalEvaluateCoreAttributes {
    /// The core attributes object contains additional data that can be used to assess the ACH return risk. Examples of data include:  `days_since_first_plaid_connection`: The number of days since the first time the Item was connected to an application via Plaid `plaid_connections_count_7d`: The number of times the Item has been connected to applications via Plaid over the past 7 days `plaid_connections_count_30d`: The number of times the Item has been connected to applications via Plaid over the past 30 days `total_plaid_connections_count`: The number of times the Item has been connected to applications via Plaid `is_savings_or_money_market_account`: Indicates whether the ACH transaction funding account is a savings/money market account  For the full list and detailed documentation of core attributes available, or to request that core attributes not be returned, contact Sales or your Plaid account manager
    pub fn new() -> SignalEvaluateCoreAttributes {
        SignalEvaluateCoreAttributes {
            unauthorized_transactions_count_7d: None,
            unauthorized_transactions_count_30d: None,
            unauthorized_transactions_count_60d: None,
            unauthorized_transactions_count_90d: None,
            nsf_overdraft_transactions_count_7d: None,
            nsf_overdraft_transactions_count_30d: None,
            nsf_overdraft_transactions_count_60d: None,
            nsf_overdraft_transactions_count_90d: None,
            days_since_first_plaid_connection: None,
            plaid_connections_count_7d: None,
            plaid_connections_count_30d: None,
            total_plaid_connections_count: None,
            is_savings_or_money_market_account: None,
            total_credit_transactions_amount_10d: None,
            total_debit_transactions_amount_10d: None,
            p50_credit_transactions_amount_28d: None,
            p50_debit_transactions_amount_28d: None,
            p95_credit_transactions_amount_28d: None,
            p95_debit_transactions_amount_28d: None,
            days_with_negative_balance_count_90d: None,
            p90_eod_balance_30d: None,
            p90_eod_balance_60d: None,
            p90_eod_balance_90d: None,
            p10_eod_balance_30d: None,
            p10_eod_balance_60d: None,
            p10_eod_balance_90d: None,
            available_balance: None,
            current_balance: None,
            balance_last_updated: None,
            phone_change_count_28d: None,
            phone_change_count_90d: None,
            email_change_count_28d: None,
            email_change_count_90d: None,
            address_change_count_28d: None,
            address_change_count_90d: None,
            plaid_non_oauth_authentication_attempts_count_3d: None,
            plaid_non_oauth_authentication_attempts_count_7d: None,
            plaid_non_oauth_authentication_attempts_count_30d: None,
            failed_plaid_non_oauth_authentication_attempts_count_3d: None,
            failed_plaid_non_oauth_authentication_attempts_count_7d: None,
            failed_plaid_non_oauth_authentication_attempts_count_30d: None,
            debit_transactions_count_10d: None,
            credit_transactions_count_10d: None,
            debit_transactions_count_30d: None,
            credit_transactions_count_30d: None,
            debit_transactions_count_60d: None,
            credit_transactions_count_60d: None,
            debit_transactions_count_90d: None,
            credit_transactions_count_90d: None,
            total_debit_transactions_amount_30d: None,
            total_credit_transactions_amount_30d: None,
            total_debit_transactions_amount_60d: None,
            total_credit_transactions_amount_60d: None,
            total_debit_transactions_amount_90d: None,
            total_credit_transactions_amount_90d: None,
            p50_eod_balance_30d: None,
            p50_eod_balance_60d: None,
            p50_eod_balance_90d: None,
            p50_eod_balance_31d_to_60d: None,
            p50_eod_balance_61d_to_90d: None,
            p90_eod_balance_31d_to_60d: None,
            p90_eod_balance_61d_to_90d: None,
            p10_eod_balance_31d_to_60d: None,
            p10_eod_balance_61d_to_90d: None,
            transactions_last_updated: None,
            is_account_closed: None,
            is_account_frozen_or_restricted: None,
            distinct_ip_addresses_count_3d: None,
            distinct_ip_addresses_count_7d: None,
            distinct_ip_addresses_count_30d: None,
            distinct_ip_addresses_count_90d: None,
            distinct_user_agents_count_3d: None,
            distinct_user_agents_count_7d: None,
            distinct_user_agents_count_30d: None,
            distinct_user_agents_count_90d: None,
            distinct_ssl_tls_connection_sessions_count_3d: None,
            distinct_ssl_tls_connection_sessions_count_7d: None,
            distinct_ssl_tls_connection_sessions_count_30d: None,
            distinct_ssl_tls_connection_sessions_count_90d: None,
            days_since_account_opening: None,
        }
    }
}


