/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CategoryInsights : Insights on a user's top personal finance categories.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CategoryInsights {
    /// List of insights of top primary personal finance categories ranked by outflow.
    #[serde(rename = "primary_category_insights", skip_serializing_if = "Option::is_none")]
    pub primary_category_insights: Option<Vec<crate::models::CategoryInsightDetails>>,
    /// List of insights of top detailed personal finance categories ranked by outflow.
    #[serde(rename = "detailed_category_insights", skip_serializing_if = "Option::is_none")]
    pub detailed_category_insights: Option<Vec<crate::models::CategoryInsightDetails>>,
}

impl CategoryInsights {
    /// Insights on a user's top personal finance categories.
    pub fn new() -> CategoryInsights {
        CategoryInsights {
            primary_category_insights: None,
            detailed_category_insights: None,
        }
    }
}


