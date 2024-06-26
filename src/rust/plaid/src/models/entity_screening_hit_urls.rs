/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EntityScreeningHitUrls : URLs associated with the entity screening hit



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityScreeningHitUrls {
    /// An 'http' or 'https' URL (must begin with either of those).
    #[serde(rename = "url")]
    pub url: String,
}

impl EntityScreeningHitUrls {
    /// URLs associated with the entity screening hit
    pub fn new(url: String) -> EntityScreeningHitUrls {
        EntityScreeningHitUrls {
            url,
        }
    }
}


