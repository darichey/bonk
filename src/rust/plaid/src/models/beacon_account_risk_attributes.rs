/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BeaconAccountRiskAttributes : The attributes object contains data that can be used to assess account risk. Examples of data include: `days_since_first_plaid_connection`: The number of days since the first time the Item was connected to an application via Plaid `plaid_connections_count_7d`: The number of times the Item has been connected to applications via Plaid over the past 7 days `plaid_connections_count_30d`: The number of times the Item has been connected to applications via Plaid over the past 30 days `total_plaid_connections_count`: The number of times the Item has been connected to applications via Plaid For the full list and detailed documentation of core attributes available, or to request that core attributes not be returned, contact Sales or your Plaid account manager



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BeaconAccountRiskAttributes {
    /// The number of days since the first time the Item was connected to an application via Plaid
    #[serde(rename = "days_since_first_plaid_connection", deserialize_with = "Option::deserialize")]
    pub days_since_first_plaid_connection: Option<i32>,
    /// Indicates if the account has been closed by the financial institution or the consumer, or is at risk of being closed
    #[serde(rename = "is_account_closed", deserialize_with = "Option::deserialize")]
    pub is_account_closed: Option<bool>,
    /// Indicates whether the account has withdrawals and transfers disabled or if access to the account is restricted. This could be due to a freeze by the credit issuer, legal restrictions (e.g., sanctions), or regulatory requirements limiting monthly withdrawals, among other reasons
    #[serde(rename = "is_account_frozen_or_restricted", deserialize_with = "Option::deserialize")]
    pub is_account_frozen_or_restricted: Option<bool>,
    /// The total number of times the item has been connected to applications via Plaid
    #[serde(rename = "total_plaid_connections_count", deserialize_with = "Option::deserialize")]
    pub total_plaid_connections_count: Option<i32>,
    /// The number of times the Item has been connected to applications via Plaid over the past 7 days
    #[serde(rename = "plaid_connections_count_7d", deserialize_with = "Option::deserialize")]
    pub plaid_connections_count_7d: Option<i32>,
    /// The number of times the Item has been connected to applications via Plaid over the past 30 days
    #[serde(rename = "plaid_connections_count_30d", deserialize_with = "Option::deserialize")]
    pub plaid_connections_count_30d: Option<i32>,
    /// The number of failed non-OAuth authentication attempts via Plaid for this bank account over the past 3 days
    #[serde(rename = "failed_plaid_non_oauth_authentication_attempts_count_3d", deserialize_with = "Option::deserialize")]
    pub failed_plaid_non_oauth_authentication_attempts_count_3d: Option<i32>,
    /// The number of non-OAuth authentication attempts via Plaid for this bank account over the past 3 days
    #[serde(rename = "plaid_non_oauth_authentication_attempts_count_3d", deserialize_with = "Option::deserialize")]
    pub plaid_non_oauth_authentication_attempts_count_3d: Option<i32>,
    /// The number of failed non-OAuth authentication attempts via Plaid for this bank account over the past 7 days
    #[serde(rename = "failed_plaid_non_oauth_authentication_attempts_count_7d", deserialize_with = "Option::deserialize")]
    pub failed_plaid_non_oauth_authentication_attempts_count_7d: Option<i32>,
    /// The number of non-OAuth authentication attempts via Plaid for this bank account over the past 7 days
    #[serde(rename = "plaid_non_oauth_authentication_attempts_count_7d", deserialize_with = "Option::deserialize")]
    pub plaid_non_oauth_authentication_attempts_count_7d: Option<i32>,
    /// The number of failed non-OAuth authentication attempts via Plaid for this bank account over the past 30 days
    #[serde(rename = "failed_plaid_non_oauth_authentication_attempts_count_30d", deserialize_with = "Option::deserialize")]
    pub failed_plaid_non_oauth_authentication_attempts_count_30d: Option<i32>,
    /// The number of non-OAuth authentication attempts via Plaid for this bank account over the past 30 days
    #[serde(rename = "plaid_non_oauth_authentication_attempts_count_30d", deserialize_with = "Option::deserialize")]
    pub plaid_non_oauth_authentication_attempts_count_30d: Option<i32>,
    /// The number of distinct IP addresses linked to the same bank account during Plaid authentication in the last 3 days
    #[serde(rename = "distinct_ip_addresses_count_3d", deserialize_with = "Option::deserialize")]
    pub distinct_ip_addresses_count_3d: Option<i32>,
    /// The number of distinct IP addresses linked to the same bank account during Plaid authentication in the last 7 days
    #[serde(rename = "distinct_ip_addresses_count_7d", deserialize_with = "Option::deserialize")]
    pub distinct_ip_addresses_count_7d: Option<i32>,
    /// The number of distinct IP addresses linked to the same bank account during Plaid authentication in the last 30 days
    #[serde(rename = "distinct_ip_addresses_count_30d", deserialize_with = "Option::deserialize")]
    pub distinct_ip_addresses_count_30d: Option<i32>,
    /// The number of distinct IP addresses linked to the same bank account during Plaid authentication in the last 90 days
    #[serde(rename = "distinct_ip_addresses_count_90d", deserialize_with = "Option::deserialize")]
    pub distinct_ip_addresses_count_90d: Option<i32>,
    /// The number of distinct user agents linked to the same bank account during Plaid authentication in the last 3 days
    #[serde(rename = "distinct_user_agents_count_3d", deserialize_with = "Option::deserialize")]
    pub distinct_user_agents_count_3d: Option<i32>,
    /// The number of distinct user agents linked to the same bank account during Plaid authentication in the last 7 days
    #[serde(rename = "distinct_user_agents_count_7d", deserialize_with = "Option::deserialize")]
    pub distinct_user_agents_count_7d: Option<i32>,
    /// The number of distinct user agents linked to the same bank account during Plaid authentication in the last 30 days
    #[serde(rename = "distinct_user_agents_count_30d", deserialize_with = "Option::deserialize")]
    pub distinct_user_agents_count_30d: Option<i32>,
    /// The number of distinct user agents linked to the same bank account during Plaid authentication in the last 90 days
    #[serde(rename = "distinct_user_agents_count_90d", deserialize_with = "Option::deserialize")]
    pub distinct_user_agents_count_90d: Option<i32>,
    /// The number of times the account's addresses on file have changed over the past 28 days
    #[serde(rename = "address_change_count_28d", deserialize_with = "Option::deserialize")]
    pub address_change_count_28d: Option<i32>,
    /// The number of times the account's email addresses on file have changed over the past 28 days
    #[serde(rename = "email_change_count_28d", deserialize_with = "Option::deserialize")]
    pub email_change_count_28d: Option<i32>,
    /// The number of times the account's phone numbers on file have changed over the past 28 days
    #[serde(rename = "phone_change_count_28d", deserialize_with = "Option::deserialize")]
    pub phone_change_count_28d: Option<i32>,
    /// The number of times the account's addresses on file have changed over the past 90 days
    #[serde(rename = "address_change_count_90d", deserialize_with = "Option::deserialize")]
    pub address_change_count_90d: Option<i32>,
    /// The number of times the account's email addresses on file have changed over the past 90 days
    #[serde(rename = "email_change_count_90d", deserialize_with = "Option::deserialize")]
    pub email_change_count_90d: Option<i32>,
    /// The number of times the account's phone numbers on file have changed over the past 90 days
    #[serde(rename = "phone_change_count_90d", deserialize_with = "Option::deserialize")]
    pub phone_change_count_90d: Option<i32>,
    /// The number of days since the bank account was opened, as reported by the financial institution
    #[serde(rename = "days_since_account_opening", deserialize_with = "Option::deserialize")]
    pub days_since_account_opening: Option<i32>,
    /// The number of days since the oldest transaction available to Plaid for this account. This measure, combined with Plaid connection history, can be used to infer the age of the account
    #[serde(rename = "days_since_first_observed_transaction", deserialize_with = "Option::deserialize")]
    pub days_since_first_observed_transaction: Option<i32>,
}

impl BeaconAccountRiskAttributes {
    /// The attributes object contains data that can be used to assess account risk. Examples of data include: `days_since_first_plaid_connection`: The number of days since the first time the Item was connected to an application via Plaid `plaid_connections_count_7d`: The number of times the Item has been connected to applications via Plaid over the past 7 days `plaid_connections_count_30d`: The number of times the Item has been connected to applications via Plaid over the past 30 days `total_plaid_connections_count`: The number of times the Item has been connected to applications via Plaid For the full list and detailed documentation of core attributes available, or to request that core attributes not be returned, contact Sales or your Plaid account manager
    pub fn new(days_since_first_plaid_connection: Option<i32>, is_account_closed: Option<bool>, is_account_frozen_or_restricted: Option<bool>, total_plaid_connections_count: Option<i32>, plaid_connections_count_7d: Option<i32>, plaid_connections_count_30d: Option<i32>, failed_plaid_non_oauth_authentication_attempts_count_3d: Option<i32>, plaid_non_oauth_authentication_attempts_count_3d: Option<i32>, failed_plaid_non_oauth_authentication_attempts_count_7d: Option<i32>, plaid_non_oauth_authentication_attempts_count_7d: Option<i32>, failed_plaid_non_oauth_authentication_attempts_count_30d: Option<i32>, plaid_non_oauth_authentication_attempts_count_30d: Option<i32>, distinct_ip_addresses_count_3d: Option<i32>, distinct_ip_addresses_count_7d: Option<i32>, distinct_ip_addresses_count_30d: Option<i32>, distinct_ip_addresses_count_90d: Option<i32>, distinct_user_agents_count_3d: Option<i32>, distinct_user_agents_count_7d: Option<i32>, distinct_user_agents_count_30d: Option<i32>, distinct_user_agents_count_90d: Option<i32>, address_change_count_28d: Option<i32>, email_change_count_28d: Option<i32>, phone_change_count_28d: Option<i32>, address_change_count_90d: Option<i32>, email_change_count_90d: Option<i32>, phone_change_count_90d: Option<i32>, days_since_account_opening: Option<i32>, days_since_first_observed_transaction: Option<i32>) -> BeaconAccountRiskAttributes {
        BeaconAccountRiskAttributes {
            days_since_first_plaid_connection,
            is_account_closed,
            is_account_frozen_or_restricted,
            total_plaid_connections_count,
            plaid_connections_count_7d,
            plaid_connections_count_30d,
            failed_plaid_non_oauth_authentication_attempts_count_3d,
            plaid_non_oauth_authentication_attempts_count_3d,
            failed_plaid_non_oauth_authentication_attempts_count_7d,
            plaid_non_oauth_authentication_attempts_count_7d,
            failed_plaid_non_oauth_authentication_attempts_count_30d,
            plaid_non_oauth_authentication_attempts_count_30d,
            distinct_ip_addresses_count_3d,
            distinct_ip_addresses_count_7d,
            distinct_ip_addresses_count_30d,
            distinct_ip_addresses_count_90d,
            distinct_user_agents_count_3d,
            distinct_user_agents_count_7d,
            distinct_user_agents_count_30d,
            distinct_user_agents_count_90d,
            address_change_count_28d,
            email_change_count_28d,
            phone_change_count_28d,
            address_change_count_90d,
            email_change_count_90d,
            phone_change_count_90d,
            days_since_account_opening,
            days_since_first_observed_transaction,
        }
    }
}

