/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Total : An object representing both the current pay period and year to date amount for a category.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Total {
    #[serde(rename = "canonical_description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub canonical_description: Option<Option<crate::models::TotalCanonicalDescription>>,
    /// Text of the line item as printed on the paystub.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "current_pay", skip_serializing_if = "Option::is_none")]
    pub current_pay: Option<crate::models::Pay>,
    #[serde(rename = "ytd_pay", skip_serializing_if = "Option::is_none")]
    pub ytd_pay: Option<crate::models::Pay>,
}

impl Total {
    /// An object representing both the current pay period and year to date amount for a category.
    pub fn new() -> Total {
        Total {
            canonical_description: None,
            description: None,
            current_pay: None,
            ytd_pay: None,
        }
    }
}


