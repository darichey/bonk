/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraLoanStatusHistoryUpdate : Contains the status and date of an update to the loan.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CraLoanStatusHistoryUpdate {
    #[serde(rename = "status")]
    pub status: crate::models::CraLoanStatus,
    /// The effective date for the status of the loan. The date should be in ISO 8601 format (YYYY-MM-DD).
    #[serde(rename = "date")]
    pub date: String,
}

impl CraLoanStatusHistoryUpdate {
    /// Contains the status and date of an update to the loan.
    pub fn new(status: crate::models::CraLoanStatus, date: String) -> CraLoanStatusHistoryUpdate {
        CraLoanStatusHistoryUpdate {
            status,
            date,
        }
    }
}

