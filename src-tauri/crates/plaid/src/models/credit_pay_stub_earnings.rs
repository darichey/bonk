/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditPayStubEarnings : An object representing both a breakdown of earnings on a pay stub and the total earnings.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditPayStubEarnings {
    #[serde(rename = "breakdown")]
    pub breakdown: Vec<crate::models::PayStubEarningsBreakdown>,
    #[serde(rename = "total")]
    pub total: crate::models::PayStubEarningsTotal,
}

impl CreditPayStubEarnings {
    /// An object representing both a breakdown of earnings on a pay stub and the total earnings.
    pub fn new(breakdown: Vec<crate::models::PayStubEarningsBreakdown>, total: crate::models::PayStubEarningsTotal) -> CreditPayStubEarnings {
        CreditPayStubEarnings {
            breakdown,
            total,
        }
    }
}


