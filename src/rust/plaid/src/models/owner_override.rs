/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OwnerOverride : Data about the owner or owners of an account. Any fields not specified will be filled in with default Sandbox information.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OwnerOverride {
    /// A list of names associated with the account by the financial institution. These should always be the names of individuals, even for business accounts. Note that the same name data will be used for all accounts associated with an Item.
    #[serde(rename = "names")]
    pub names: Vec<String>,
    /// A list of phone numbers associated with the account.
    #[serde(rename = "phone_numbers")]
    pub phone_numbers: Vec<crate::models::PhoneNumber>,
    /// A list of email addresses associated with the account.
    #[serde(rename = "emails")]
    pub emails: Vec<crate::models::Email>,
    /// Data about the various addresses associated with the account.
    #[serde(rename = "addresses")]
    pub addresses: Vec<crate::models::Address>,
}

impl OwnerOverride {
    /// Data about the owner or owners of an account. Any fields not specified will be filled in with default Sandbox information.
    pub fn new(names: Vec<String>, phone_numbers: Vec<crate::models::PhoneNumber>, emails: Vec<crate::models::Email>, addresses: Vec<crate::models::Address>) -> OwnerOverride {
        OwnerOverride {
            names,
            phone_numbers,
            emails,
            addresses,
        }
    }
}


