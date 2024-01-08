/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankEmployer : Object containing employer data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditBankEmployer {
    /// Name of the employer.
    #[serde(rename = "name")]
    pub name: String,
}

impl CreditBankEmployer {
    /// Object containing employer data.
    pub fn new(name: String) -> CreditBankEmployer {
        CreditBankEmployer {
            name,
        }
    }
}


