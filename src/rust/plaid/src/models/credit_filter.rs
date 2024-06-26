/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditFilter : A filter to apply to `credit`-type accounts



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditFilter {
    /// An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema). 
    #[serde(rename = "account_subtypes")]
    pub account_subtypes: Vec<crate::models::CreditAccountSubtype>,
}

impl CreditFilter {
    /// A filter to apply to `credit`-type accounts
    pub fn new(account_subtypes: Vec<crate::models::CreditAccountSubtype>) -> CreditFilter {
        CreditFilter {
            account_subtypes,
        }
    }
}


