/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PendingDisconnectWebhookReason : Reason why the item is about to be disconnected. `INSTITUTION_MIGRATION`: The institution is moving to API or to a different integration. For example, this can occur when an institution moves from a non-OAuth integration to an OAuth integration. `INSTITUTION_TOKEN_EXPIRATION`: The consent on an Item associated with a US or CA institution is about to expire.

/// Reason why the item is about to be disconnected. `INSTITUTION_MIGRATION`: The institution is moving to API or to a different integration. For example, this can occur when an institution moves from a non-OAuth integration to an OAuth integration. `INSTITUTION_TOKEN_EXPIRATION`: The consent on an Item associated with a US or CA institution is about to expire.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PendingDisconnectWebhookReason {
    #[serde(rename = "INSTITUTION_MIGRATION")]
    Migration,
    #[serde(rename = "INSTITUTION_TOKEN_EXPIRATION")]
    TokenExpiration,

}

impl ToString for PendingDisconnectWebhookReason {
    fn to_string(&self) -> String {
        match self {
            Self::Migration => String::from("INSTITUTION_MIGRATION"),
            Self::TokenExpiration => String::from("INSTITUTION_TOKEN_EXPIRATION"),
        }
    }
}

impl Default for PendingDisconnectWebhookReason {
    fn default() -> PendingDisconnectWebhookReason {
        Self::Migration
    }
}



