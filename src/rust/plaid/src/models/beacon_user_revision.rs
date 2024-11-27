/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BeaconUserRevision : A Beacon User Revision identifies a Beacon User at some point in its revision history.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BeaconUserRevision {
    /// ID of the associated Beacon User.
    #[serde(rename = "id")]
    pub id: String,
    /// The `version` field begins with 1 and increments with each subsequent revision.
    #[serde(rename = "version")]
    pub version: i32,
}

impl BeaconUserRevision {
    /// A Beacon User Revision identifies a Beacon User at some point in its revision history.
    pub fn new(id: String, version: i32) -> BeaconUserRevision {
        BeaconUserRevision {
            id,
            version,
        }
    }
}

