/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DashboardUserGetResponse : Account information associated with a team member with access to the Plaid dashboard.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DashboardUserGetResponse {
    /// ID of the associated user. To retrieve the email address or other details of the person corresponding to this id, use `/dashboard_user/get`.
    #[serde(rename = "id")]
    pub id: String,
    /// An ISO8601 formatted timestamp.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// A valid email address. Must not have leading or trailing spaces and address must be RFC compliant. For more information, see [RFC 3696](https://datatracker.ietf.org/doc/html/rfc3696).
    #[serde(rename = "email_address")]
    pub email_address: String,
    #[serde(rename = "status")]
    pub status: crate::models::DashboardUserStatus,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl DashboardUserGetResponse {
    /// Account information associated with a team member with access to the Plaid dashboard.
    pub fn new(id: String, created_at: String, email_address: String, status: crate::models::DashboardUserStatus, request_id: String) -> DashboardUserGetResponse {
        DashboardUserGetResponse {
            id,
            created_at,
            email_address,
            status,
            request_id,
        }
    }
}


