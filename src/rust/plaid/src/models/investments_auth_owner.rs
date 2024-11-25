/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InvestmentsAuthOwner : Information on the ownership of an investments account



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvestmentsAuthOwner {
    /// The ID of the account that this identity information pertains to
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// A list of names associated with the account by the financial institution. In the case of a joint account, Plaid will make a best effort to report the names of all account holders.  If an Item contains multiple accounts with different owner names, some institutions will report all names associated with the Item in each account's `names` array.
    #[serde(rename = "names", skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
}

impl InvestmentsAuthOwner {
    /// Information on the ownership of an investments account
    pub fn new() -> InvestmentsAuthOwner {
        InvestmentsAuthOwner {
            account_id: None,
            names: None,
        }
    }
}


