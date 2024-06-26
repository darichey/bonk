/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsUserInsightsGetResponse : TransactionsUserInsightsGetResponse defines the response schema for `/beta/transactions/user_insights/v1/get`.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionsUserInsightsGetResponse {
    #[serde(rename = "user_data_overview")]
    pub user_data_overview: crate::models::UserDataOverview,
    #[serde(rename = "counterparty_insights", skip_serializing_if = "Option::is_none")]
    pub counterparty_insights: Option<crate::models::CounterpartyInsights>,
    #[serde(rename = "category_insights", skip_serializing_if = "Option::is_none")]
    pub category_insights: Option<crate::models::CategoryInsights>,
    #[serde(rename = "recurring_transactions", skip_serializing_if = "Option::is_none")]
    pub recurring_transactions: Option<crate::models::RecurringTransactions>,
}

impl TransactionsUserInsightsGetResponse {
    /// TransactionsUserInsightsGetResponse defines the response schema for `/beta/transactions/user_insights/v1/get`.
    pub fn new(user_data_overview: crate::models::UserDataOverview) -> TransactionsUserInsightsGetResponse {
        TransactionsUserInsightsGetResponse {
            user_data_overview,
            counterparty_insights: None,
            category_insights: None,
            recurring_transactions: None,
        }
    }
}


