/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RecurringTransactions : Insights object for recurring transactions for `/beta/transactions/user_insights/v1/get` endpoint



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecurringTransactions {
    /// An array of inflow transaction streams (e.g., income).
    #[serde(rename = "inflow_streams")]
    pub inflow_streams: Vec<crate::models::RecurringInsightsStream>,
    /// An array of outflow transaction streams (e.g., subscriptions, bills, loan payments).
    #[serde(rename = "outflow_streams")]
    pub outflow_streams: Vec<crate::models::RecurringInsightsStream>,
}

impl RecurringTransactions {
    /// Insights object for recurring transactions for `/beta/transactions/user_insights/v1/get` endpoint
    pub fn new(inflow_streams: Vec<crate::models::RecurringInsightsStream>, outflow_streams: Vec<crate::models::RecurringInsightsStream>) -> RecurringTransactions {
        RecurringTransactions {
            inflow_streams,
            outflow_streams,
        }
    }
}

