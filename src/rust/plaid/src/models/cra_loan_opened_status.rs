/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraLoanOpenedStatus : Contains the status and date information of the loan when registering.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CraLoanOpenedStatus {
    #[serde(rename = "status")]
    pub status: crate::models::CraLoanStatus,
    /// The effective date for the status of the loan. The date should be in ISO 8601 format (YYYY-MM-DD).
    #[serde(rename = "date")]
    pub date: String,
}

impl CraLoanOpenedStatus {
    /// Contains the status and date information of the loan when registering.
    pub fn new(status: crate::models::CraLoanStatus, date: String) -> CraLoanOpenedStatus {
        CraLoanOpenedStatus {
            status,
            date,
        }
    }
}


