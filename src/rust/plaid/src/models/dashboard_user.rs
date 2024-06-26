/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DashboardUser : Account information associated with a team member with access to the Plaid dashboard.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardUser {
    /// ID of the associated user.
    #[serde(rename = "id")]
    pub id: String,
    /// An ISO8601 formatted timestamp.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// A valid email address.
    #[serde(rename = "email_address")]
    pub email_address: String,
    #[serde(rename = "status")]
    pub status: crate::models::DashboardUserStatus,
}

impl DashboardUser {
    /// Account information associated with a team member with access to the Plaid dashboard.
    pub fn new(id: String, created_at: String, email_address: String, status: crate::models::DashboardUserStatus) -> DashboardUser {
        DashboardUser {
            id,
            created_at,
            email_address,
            status,
        }
    }
}


