/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkSessionItemAddResult : The details of an Item add in Link.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkSessionItemAddResult {
    /// Returned once a user has successfully linked their Item.
    #[serde(rename = "public_token")]
    pub public_token: String,
    /// A list of accounts attached to the connected Item. If Account Select is enabled via the developer dashboard, `accounts` will only include selected accounts.
    #[serde(rename = "accounts")]
    pub accounts: Vec<crate::models::LinkSessionSuccessMetadataAccount>,
    #[serde(rename = "institution", deserialize_with = "Option::deserialize")]
    pub institution: Option<Box<crate::models::LinkSessionSuccessMetadataInstitution>>,
}

impl LinkSessionItemAddResult {
    /// The details of an Item add in Link.
    pub fn new(public_token: String, accounts: Vec<crate::models::LinkSessionSuccessMetadataAccount>, institution: Option<crate::models::LinkSessionSuccessMetadataInstitution>) -> LinkSessionItemAddResult {
        LinkSessionItemAddResult {
            public_token,
            accounts,
            institution: if let Some(x) = institution {Some(Box::new(x))} else {None},
        }
    }
}

