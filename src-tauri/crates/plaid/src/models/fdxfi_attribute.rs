/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FdxfiAttribute : Financial Institution provider-specific attribute



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FdxfiAttribute {
    /// Name of attribute
    #[serde(rename = "name")]
    pub name: String,
    /// Value of attribute
    #[serde(rename = "value")]
    pub value: String,
}

impl FdxfiAttribute {
    /// Financial Institution provider-specific attribute
    pub fn new(name: String, value: String) -> FdxfiAttribute {
        FdxfiAttribute {
            name,
            value,
        }
    }
}


