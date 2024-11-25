/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaystubOverrideEmployer : The employer on the paystub.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaystubOverrideEmployer {
    /// The name of the employer.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Box<crate::models::PaystubOverrideEmployerAddress>>,
}

impl PaystubOverrideEmployer {
    /// The employer on the paystub.
    pub fn new() -> PaystubOverrideEmployer {
        PaystubOverrideEmployer {
            name: None,
            address: None,
        }
    }
}


