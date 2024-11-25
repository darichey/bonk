/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WatchlistScreeningIndividualUpdateRequestResettableField : The name of a field that can be reset back to null

/// The name of a field that can be reset back to null
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WatchlistScreeningIndividualUpdateRequestResettableField {
    #[serde(rename = "assignee")]
    Assignee,

}

impl ToString for WatchlistScreeningIndividualUpdateRequestResettableField {
    fn to_string(&self) -> String {
        match self {
            Self::Assignee => String::from("assignee"),
        }
    }
}

impl Default for WatchlistScreeningIndividualUpdateRequestResettableField {
    fn default() -> WatchlistScreeningIndividualUpdateRequestResettableField {
        Self::Assignee
    }
}




