/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Issue : Information on an issue encountered with financial institutions interactions with financial institutions during Linking.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Issue {
    /// The unique identifier of the issue.
    #[serde(rename = "issue_id", skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<String>,
    /// A list of names of the financial institutions affected.
    #[serde(rename = "institution_names", skip_serializing_if = "Option::is_none")]
    pub institution_names: Option<Vec<String>>,
    /// A list of ids of the financial institutions affected.
    #[serde(rename = "institution_ids", skip_serializing_if = "Option::is_none")]
    pub institution_ids: Option<Vec<String>>,
    /// The creation time of the record tracking this issue.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// A simple summary of the error for the end user.
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A more detailed description for the customer.
    #[serde(rename = "detailed_description", skip_serializing_if = "Option::is_none")]
    pub detailed_description: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::IssuesStatus>,
}

impl Issue {
    /// Information on an issue encountered with financial institutions interactions with financial institutions during Linking.
    pub fn new() -> Issue {
        Issue {
            issue_id: None,
            institution_names: None,
            institution_ids: None,
            created_at: None,
            summary: None,
            detailed_description: None,
            status: None,
        }
    }
}


