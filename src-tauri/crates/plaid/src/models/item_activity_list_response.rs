/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemActivityListResponse : Describes a historical log of user consent events.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemActivityListResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
    /// A list of activities.
    #[serde(rename = "activities")]
    pub activities: Vec<crate::models::Activity>,
    /// An array of objects containing timestamps for the last time each data type was accessed per application.
    #[serde(rename = "last_data_access_times")]
    pub last_data_access_times: Vec<crate::models::LastDataAccessTimes>,
    /// Cursor used for pagination.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ItemActivityListResponse {
    /// Describes a historical log of user consent events.
    pub fn new(request_id: String, activities: Vec<crate::models::Activity>, last_data_access_times: Vec<crate::models::LastDataAccessTimes>) -> ItemActivityListResponse {
        ItemActivityListResponse {
            request_id,
            activities,
            last_data_access_times,
            cursor: None,
        }
    }
}


