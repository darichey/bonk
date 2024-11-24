/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConsentEvent : Describes a consent event.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConsentEvent {
    /// The Plaid Item ID. The `item_id` is always unique; linking the same account at the same institution twice will result in two Items with different `item_id` values. Like all Plaid identifiers, the `item_id` is case-sensitive.
    #[serde(rename = "item_id", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    /// The date and time when the consent event occurred, in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "event_type", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<crate::models::ConsentEventType>,
    #[serde(rename = "event_code", skip_serializing_if = "Option::is_none")]
    pub event_code: Option<crate::models::ConsentEventCode>,
    /// Unique identifier for the institution associated with the Item. Field is `null` for Items created via Same Day Micro-deposits.
    #[serde(rename = "institution_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<Option<String>>,
    /// The full name of the institution associated with the Item. Field is `null` for Items created via Same Day Micro-deposits.
    #[serde(rename = "institution_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<Option<String>>,
    #[serde(rename = "initiator", skip_serializing_if = "Option::is_none")]
    pub initiator: Option<crate::models::ConsentEventInitiator>,
    /// A list of strings containing the full list of use cases the end user has consented to for the Item.  See the [full list](/docs/link/data-transparency-messaging-migration-guide/#updating-link-customizations) of use cases.
    #[serde(rename = "consented_use_cases", skip_serializing_if = "Option::is_none")]
    pub consented_use_cases: Option<Vec<String>>,
    /// A list of strings containing the full list of data scopes the end user has consented to for the Item. These correspond to consented products; see the [full mapping](/docs/link/data-transparency-messaging-migration-guide/#data-scopes-by-product) of data scopes and products.
    #[serde(rename = "consented_data_scopes", skip_serializing_if = "Option::is_none")]
    pub consented_data_scopes: Option<Vec<String>>,
    /// An array containing the accounts associated with the Item for which authorizations are granted.
    #[serde(rename = "consented_accounts", skip_serializing_if = "Option::is_none")]
    pub consented_accounts: Option<Vec<crate::models::ConsentedAccount>>,
}

impl ConsentEvent {
    /// Describes a consent event.
    pub fn new() -> ConsentEvent {
        ConsentEvent {
            item_id: None,
            created_at: None,
            event_type: None,
            event_code: None,
            institution_id: None,
            institution_name: None,
            initiator: None,
            consented_use_cases: None,
            consented_data_scopes: None,
            consented_accounts: None,
        }
    }
}


