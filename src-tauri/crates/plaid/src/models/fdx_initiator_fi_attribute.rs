/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FdxInitiatorFiAttribute : Initiator Fi Attribute



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FdxInitiatorFiAttribute {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<crate::models::FdxPartyType>,
}

impl FdxInitiatorFiAttribute {
    /// Initiator Fi Attribute
    pub fn new() -> FdxInitiatorFiAttribute {
        FdxInitiatorFiAttribute {
            name: None,
            value: None,
        }
    }
}


