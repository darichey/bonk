/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditPayStubEmployee : Data about the employee.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditPayStubEmployee {
    #[serde(rename = "address")]
    pub address: crate::models::CreditPayStubAddress,
    /// The name of the employee.
    #[serde(rename = "name", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
    /// Marital status of the employee - either `SINGLE` or `MARRIED` or `NOT LISTED`.
    #[serde(rename = "marital_status", deserialize_with = "Option::deserialize")]
    pub marital_status: Option<String>,
    #[serde(rename = "taxpayer_id")]
    pub taxpayer_id: crate::models::PayStubTaxpayerId,
}

impl CreditPayStubEmployee {
    /// Data about the employee.
    pub fn new(address: crate::models::CreditPayStubAddress, name: Option<String>, marital_status: Option<String>, taxpayer_id: crate::models::PayStubTaxpayerId) -> CreditPayStubEmployee {
        CreditPayStubEmployee {
            address,
            name,
            marital_status,
            taxpayer_id,
        }
    }
}


