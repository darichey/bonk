/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BaseReportAccount : Base Report information about an account



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseReportAccount {
    #[serde(rename = "balances")]
    pub balances: Box<crate::models::BaseReportAccountBalances>,
    /// The last 2-4 alphanumeric characters of an account's official account number. Note that the mask may be non-unique between an Item's accounts, and it may also not match the mask that the bank displays to the user.
    #[serde(rename = "mask", deserialize_with = "Option::deserialize")]
    pub mask: Option<String>,
    /// The name of the account, either assigned by the user or by the financial institution itself
    #[serde(rename = "name")]
    pub name: String,
    /// The official name of the account as given by the financial institution
    #[serde(rename = "official_name", deserialize_with = "Option::deserialize")]
    pub official_name: Option<String>,
    #[serde(rename = "type")]
    pub r#type: crate::models::AccountType,
    #[serde(rename = "subtype", deserialize_with = "Option::deserialize")]
    pub subtype: Option<crate::models::AccountSubtype>,
    /// The duration of transaction history available for this Item, typically defined as the time since the date of the earliest transaction in that account. Only returned by Base Report endpoints.
    #[serde(rename = "days_available")]
    pub days_available: f32,
    /// Transaction history associated with the account. Only returned by Base Report endpoints. Transaction history returned by endpoints such as `/transactions/get` or `/investments/transactions/get` will be returned in the top-level `transactions` field instead.
    #[serde(rename = "transactions")]
    pub transactions: Vec<crate::models::BaseReportTransaction>,
    /// Data returned by the financial institution about the account owner or owners. For business accounts, the name reported may be either the name of the individual or the name of the business, depending on the institution. Multiple owners on a single account will be represented in the same `owner` object, not in multiple owner objects within the array.
    #[serde(rename = "owners")]
    pub owners: Vec<crate::models::Owner>,
    #[serde(rename = "ownership_type", deserialize_with = "Option::deserialize")]
    pub ownership_type: Option<crate::models::OwnershipType>,
    /// Calculated data about the historical balances on the account. Only returned by Base Report endpoints and currently not supported by `brokerage` or `investment` accounts.
    #[serde(rename = "historical_balances")]
    pub historical_balances: Vec<crate::models::HistoricalBalance>,
    #[serde(rename = "account_insights", skip_serializing_if = "Option::is_none")]
    pub account_insights: Option<crate::models::BaseReportAccountInsights>,
}

impl BaseReportAccount {
    /// Base Report information about an account
    pub fn new(balances: crate::models::BaseReportAccountBalances, mask: Option<String>, name: String, official_name: Option<String>, r#type: crate::models::AccountType, subtype: Option<crate::models::AccountSubtype>, days_available: f32, transactions: Vec<crate::models::BaseReportTransaction>, owners: Vec<crate::models::Owner>, ownership_type: Option<crate::models::OwnershipType>, historical_balances: Vec<crate::models::HistoricalBalance>) -> BaseReportAccount {
        BaseReportAccount {
            balances: Box::new(balances),
            mask,
            name,
            official_name,
            r#type,
            subtype,
            days_available,
            transactions,
            owners,
            ownership_type,
            historical_balances,
            account_insights: None,
        }
    }
}


