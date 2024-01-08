/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OtherAccountSubtype : Valid account subtypes for other accounts. For a list containing descriptions of each subtype, see [Account schemas](https://plaid.com/docs/api/accounts/#StandaloneAccountType-other).

/// Valid account subtypes for other accounts. For a list containing descriptions of each subtype, see [Account schemas](https://plaid.com/docs/api/accounts/#StandaloneAccountType-other).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OtherAccountSubtype {
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "all")]
    All,

}

impl ToString for OtherAccountSubtype {
    fn to_string(&self) -> String {
        match self {
            Self::Other => String::from("other"),
            Self::All => String::from("all"),
        }
    }
}

impl Default for OtherAccountSubtype {
    fn default() -> OtherAccountSubtype {
        Self::Other
    }
}




