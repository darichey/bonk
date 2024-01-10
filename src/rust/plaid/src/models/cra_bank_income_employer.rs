/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CraBankIncomeEmployer : The object containing employer data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CraBankIncomeEmployer {
    /// The name of the employer.
    #[serde(rename = "name", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
}

impl CraBankIncomeEmployer {
    /// The object containing employer data.
    pub fn new(name: Option<String>) -> CraBankIncomeEmployer {
        CraBankIncomeEmployer {
            name,
        }
    }
}

