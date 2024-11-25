/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AccountVerificationInsightsPreviousReturns : Information about known ACH returns for the account and routing number.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountVerificationInsightsPreviousReturns {
    /// Indicates whether Plaid's data sources include a known administrative ACH return for account and routing number.
    #[serde(rename = "has_previous_administrative_return")]
    pub has_previous_administrative_return: bool,
}

impl AccountVerificationInsightsPreviousReturns {
    /// Information about known ACH returns for the account and routing number.
    pub fn new(has_previous_administrative_return: bool) -> AccountVerificationInsightsPreviousReturns {
        AccountVerificationInsightsPreviousReturns {
            has_previous_administrative_return,
        }
    }
}


