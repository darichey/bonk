/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FdxParty : FDX Participant - an entity or person that is a part of a FDX API transaction



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FdxParty {
    /// Human recognizable common name
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::FdxPartyType,
    /// URI for party, where an end user could learn more about the company or application involved in the data sharing chain
    #[serde(rename = "homeUri", skip_serializing_if = "Option::is_none")]
    pub home_uri: Option<String>,
    /// URI for a logo asset to be displayed to the end user
    #[serde(rename = "logoUri", skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<String>,
    #[serde(rename = "registry", skip_serializing_if = "Option::is_none")]
    pub registry: Option<crate::models::FdxPartyRegistry>,
    /// Registered name of party
    #[serde(rename = "registeredEntityName", skip_serializing_if = "Option::is_none")]
    pub registered_entity_name: Option<String>,
    /// Registered id of party
    #[serde(rename = "registeredEntityId", skip_serializing_if = "Option::is_none")]
    pub registered_entity_id: Option<String>,
}

impl FdxParty {
    /// FDX Participant - an entity or person that is a part of a FDX API transaction
    pub fn new(name: String, r#type: crate::models::FdxPartyType) -> FdxParty {
        FdxParty {
            name,
            r#type,
            home_uri: None,
            logo_uri: None,
            registry: None,
            registered_entity_name: None,
            registered_entity_id: None,
        }
    }
}


