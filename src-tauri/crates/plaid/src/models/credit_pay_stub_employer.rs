/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditPayStubEmployer : Information about the employer on the pay stub.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditPayStubEmployer {
    #[serde(rename = "address")]
    pub address: crate::models::CreditPayStubAddress,
    /// The name of the employer on the pay stub.
    #[serde(rename = "name", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
}

impl CreditPayStubEmployer {
    /// Information about the employer on the pay stub.
    pub fn new(address: crate::models::CreditPayStubAddress, name: Option<String>) -> CreditPayStubEmployer {
        CreditPayStubEmployer {
            address,
            name,
        }
    }
}

