/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Employer : Data about the employer.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Employer {
    /// Plaid's unique identifier for the employer.
    #[serde(rename = "employer_id")]
    pub employer_id: String,
    /// The name of the employer
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "address", deserialize_with = "Option::deserialize")]
    pub address: Option<Box<crate::models::AddressDataNullable>>,
    /// A number from 0 to 1 indicating Plaid's level of confidence in the pairing between the employer and the institution (not yet implemented).
    #[serde(rename = "confidence_score")]
    pub confidence_score: f64,
}

impl Employer {
    /// Data about the employer.
    pub fn new(employer_id: String, name: String, address: Option<crate::models::AddressDataNullable>, confidence_score: f64) -> Employer {
        Employer {
            employer_id,
            name,
            address: if let Some(x) = address {Some(Box::new(x))} else {None},
            confidence_score,
        }
    }
}

