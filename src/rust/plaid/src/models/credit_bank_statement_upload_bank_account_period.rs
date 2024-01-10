/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankStatementUploadBankAccountPeriod : An object containing data on the overall period of the statement.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditBankStatementUploadBankAccountPeriod {
    /// The start date of the statement period in ISO 8601 format (YYYY-MM-DD).
    #[serde(rename = "start_date", deserialize_with = "Option::deserialize")]
    pub start_date: Option<String>,
    /// The end date of the statement period in ISO 8601 format (YYYY-MM-DD).
    #[serde(rename = "end_date", deserialize_with = "Option::deserialize")]
    pub end_date: Option<String>,
    /// The starting balance of the bank account for the period.
    #[serde(rename = "starting_balance", deserialize_with = "Option::deserialize")]
    pub starting_balance: Option<f32>,
    /// The ending balance of the bank account for the period.
    #[serde(rename = "ending_balance", deserialize_with = "Option::deserialize")]
    pub ending_balance: Option<f32>,
}

impl CreditBankStatementUploadBankAccountPeriod {
    /// An object containing data on the overall period of the statement.
    pub fn new(start_date: Option<String>, end_date: Option<String>, starting_balance: Option<f32>, ending_balance: Option<f32>) -> CreditBankStatementUploadBankAccountPeriod {
        CreditBankStatementUploadBankAccountPeriod {
            start_date,
            end_date,
            starting_balance,
            ending_balance,
        }
    }
}

