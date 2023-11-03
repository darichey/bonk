/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Meta : Allows specifying the metadata of the test account



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Meta {
    /// The account's name
    #[serde(rename = "name")]
    pub name: String,
    /// The account's official name
    #[serde(rename = "official_name")]
    pub official_name: String,
    /// The account's limit
    #[serde(rename = "limit")]
    pub limit: f64,
    /// The account's mask. Should be an empty string or a string of 2-4 alphanumeric characters. This allows you to model a mask which does not match the account number (such as with a virtual account number).
    #[serde(rename = "mask")]
    pub mask: String,
}

impl Meta {
    /// Allows specifying the metadata of the test account
    pub fn new(name: String, official_name: String, limit: f64, mask: String) -> Meta {
        Meta {
            name,
            official_name,
            limit,
            mask,
        }
    }
}


