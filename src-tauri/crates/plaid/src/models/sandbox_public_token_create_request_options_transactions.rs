/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxPublicTokenCreateRequestOptionsTransactions : An optional set of parameters corresponding to transactions options.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SandboxPublicTokenCreateRequestOptionsTransactions {
    /// The earliest date for which to fetch transaction history. Dates should be formatted as YYYY-MM-DD.
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// The most recent date for which to fetch transaction history. Dates should be formatted as YYYY-MM-DD.
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

impl SandboxPublicTokenCreateRequestOptionsTransactions {
    /// An optional set of parameters corresponding to transactions options.
    pub fn new() -> SandboxPublicTokenCreateRequestOptionsTransactions {
        SandboxPublicTokenCreateRequestOptionsTransactions {
            start_date: None,
            end_date: None,
        }
    }
}


