/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EntityScreeningHitNames : Name information for the associated entity watchlist hit



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EntityScreeningHitNames {
    /// The full name of the entity.
    #[serde(rename = "full")]
    pub full: String,
    /// Primary names are those most commonly used to refer to this entity. Only one name will ever be marked as primary.
    #[serde(rename = "is_primary")]
    pub is_primary: bool,
    #[serde(rename = "weak_alias_determination")]
    pub weak_alias_determination: crate::models::WeakAliasDetermination,
}

impl EntityScreeningHitNames {
    /// Name information for the associated entity watchlist hit
    pub fn new(full: String, is_primary: bool, weak_alias_determination: crate::models::WeakAliasDetermination) -> EntityScreeningHitNames {
        EntityScreeningHitNames {
            full,
            is_primary,
            weak_alias_determination,
        }
    }
}


