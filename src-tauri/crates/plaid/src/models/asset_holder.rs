/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AssetHolder : Documentation not found in the MISMO model viewer and not provided by Freddie Mac.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetHolder {
    #[serde(rename = "NAME")]
    pub name: crate::models::AssetHolderName,
}

impl AssetHolder {
    /// Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    pub fn new(name: crate::models::AssetHolderName) -> AssetHolder {
        AssetHolder {
            name,
        }
    }
}


