/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Activity : Describes a consent activity.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Activity {
    #[serde(rename = "activity")]
    pub activity: crate::models::ActivityType,
    /// The date this activity was initiated [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC.
    #[serde(rename = "initiated_date")]
    pub initiated_date: String,
    /// A unique identifier for the activity
    #[serde(rename = "id")]
    pub id: String,
    /// Application ID of the client who initiated the activity.
    #[serde(rename = "initiator")]
    pub initiator: String,
    #[serde(rename = "state")]
    pub state: crate::models::ActionState,
    /// This field will map to the application ID that is returned from /item/application/list, or provided to the institution in an oauth redirect.
    #[serde(rename = "target_application_id", skip_serializing_if = "Option::is_none")]
    pub target_application_id: Option<String>,
    #[serde(rename = "scopes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Option<Box<crate::models::ScopesNullable>>>,
}

impl Activity {
    /// Describes a consent activity.
    pub fn new(activity: crate::models::ActivityType, initiated_date: String, id: String, initiator: String, state: crate::models::ActionState) -> Activity {
        Activity {
            activity,
            initiated_date,
            id,
            initiator,
            state,
            target_application_id: None,
            scopes: None,
        }
    }
}


