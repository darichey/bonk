/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Deductions : An object with the deduction information found on a paystub.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Deductions {
    #[serde(rename = "subtotals", skip_serializing_if = "Option::is_none")]
    pub subtotals: Option<Vec<crate::models::Total>>,
    #[serde(rename = "breakdown")]
    pub breakdown: Vec<crate::models::DeductionsBreakdown>,
    #[serde(rename = "totals", skip_serializing_if = "Option::is_none")]
    pub totals: Option<Vec<crate::models::Total>>,
    #[serde(rename = "total")]
    pub total: crate::models::DeductionsTotal,
}

impl Deductions {
    /// An object with the deduction information found on a paystub.
    pub fn new(breakdown: Vec<crate::models::DeductionsBreakdown>, total: crate::models::DeductionsTotal) -> Deductions {
        Deductions {
            subtotals: None,
            breakdown,
            totals: None,
            total,
        }
    }
}


