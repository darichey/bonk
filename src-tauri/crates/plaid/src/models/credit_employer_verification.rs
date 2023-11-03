/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditEmployerVerification : An object containing employer data.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditEmployerVerification {
    /// Name of employer.
    #[serde(rename = "name", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
}

impl CreditEmployerVerification {
    /// An object containing employer data.
    pub fn new(name: Option<String>) -> CreditEmployerVerification {
        CreditEmployerVerification {
            name,
        }
    }
}


