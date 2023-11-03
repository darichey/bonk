/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CounterpartyInsights : Insights around a user's counterparties



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CounterpartyInsights {
    /// Insights related to a user’s transactions with other financial institutions, including detected account types.
    #[serde(rename = "financial_institution_insights", skip_serializing_if = "Option::is_none")]
    pub financial_institution_insights: Option<Vec<crate::models::FinancialInstitutionInsights>>,
    /// Insights about a user’s top merchants, ranked by spend.
    #[serde(rename = "merchant_insights", skip_serializing_if = "Option::is_none")]
    pub merchant_insights: Option<Vec<crate::models::MerchantInsights>>,
}

impl CounterpartyInsights {
    /// Insights around a user's counterparties
    pub fn new() -> CounterpartyInsights {
        CounterpartyInsights {
            financial_institution_insights: None,
            merchant_insights: None,
        }
    }
}


