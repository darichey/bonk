/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaystubOverrideEmployeeAddress : The address of the employee.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaystubOverrideEmployeeAddress {
    /// The full city name.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The region or state Example: `\"NC\"`
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The full street address Example: `\"564 Main Street, APT 15\"`
    #[serde(rename = "street", skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    /// 5 digit postal code.
    #[serde(rename = "postal_code", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The country of the address.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl PaystubOverrideEmployeeAddress {
    /// The address of the employee.
    pub fn new() -> PaystubOverrideEmployeeAddress {
        PaystubOverrideEmployeeAddress {
            city: None,
            region: None,
            street: None,
            postal_code: None,
            country: None,
        }
    }
}


