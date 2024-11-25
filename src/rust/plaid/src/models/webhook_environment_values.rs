/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WebhookEnvironmentValues : The Plaid environment the webhook was sent from

/// The Plaid environment the webhook was sent from
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WebhookEnvironmentValues {
    #[serde(rename = "sandbox")]
    Sandbox,
    #[serde(rename = "production")]
    Production,

}

impl ToString for WebhookEnvironmentValues {
    fn to_string(&self) -> String {
        match self {
            Self::Sandbox => String::from("sandbox"),
            Self::Production => String::from("production"),
        }
    }
}

impl Default for WebhookEnvironmentValues {
    fn default() -> WebhookEnvironmentValues {
        Self::Sandbox
    }
}




