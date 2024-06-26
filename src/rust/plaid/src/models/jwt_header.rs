/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JwtHeader : A JWT Header, used for webhook validation



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JwtHeader {
    #[serde(rename = "id")]
    pub id: String,
}

impl JwtHeader {
    /// A JWT Header, used for webhook validation
    pub fn new(id: String) -> JwtHeader {
        JwtHeader {
            id,
        }
    }
}


