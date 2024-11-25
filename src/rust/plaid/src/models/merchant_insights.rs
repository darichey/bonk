/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MerchantInsights : Insights into a user’s top merchants.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MerchantInsights {
    /// The counterparty name.
    #[serde(rename = "name")]
    pub name: String,
    /// A unique, stable, Plaid-generated id that maps to the merchant.
    #[serde(rename = "entity_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<Option<String>>,
    /// The website associated with the merchant.
    #[serde(rename = "website", deserialize_with = "Option::deserialize")]
    pub website: Option<String>,
    /// The number of transactions associated with merchant of this type.
    #[serde(rename = "transaction_count")]
    pub transaction_count: i32,
    /// The primary personal finance category associated with this merchant.
    #[serde(rename = "personal_finance_category_primary", deserialize_with = "Option::deserialize")]
    pub personal_finance_category_primary: Option<String>,
    /// The detailed personal finance category associated with this merchant.
    #[serde(rename = "personal_finance_category_detailed", deserialize_with = "Option::deserialize")]
    pub personal_finance_category_detailed: Option<String>,
    /// Sum of outflow amounts.
    #[serde(rename = "total_outflows")]
    pub total_outflows: f64,
    /// Sum of inflow amounts.
    #[serde(rename = "total_inflows")]
    pub total_inflows: f64,
}

impl MerchantInsights {
    /// Insights into a user’s top merchants.
    pub fn new(name: String, website: Option<String>, transaction_count: i32, personal_finance_category_primary: Option<String>, personal_finance_category_detailed: Option<String>, total_outflows: f64, total_inflows: f64) -> MerchantInsights {
        MerchantInsights {
            name,
            entity_id: None,
            website,
            transaction_count,
            personal_finance_category_primary,
            personal_finance_category_detailed,
            total_outflows,
            total_inflows,
        }
    }
}


