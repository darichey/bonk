/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProductPermissionsRequiredIdentityWebhook : Fired when an `ACCESS_NOT_GRANTED` error is hit for Identity. The error can be resolved by putting the user through update mode with `identity` in the `products` array, as well as through the limited beta for update mode Identity product validations.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductPermissionsRequiredIdentityWebhook {
    /// `IDENTITY`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `PRODUCT_PERMISSIONS_REQUIRED`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl ProductPermissionsRequiredIdentityWebhook {
    /// Fired when an `ACCESS_NOT_GRANTED` error is hit for Identity. The error can be resolved by putting the user through update mode with `identity` in the `products` array, as well as through the limited beta for update mode Identity product validations.
    pub fn new(webhook_type: String, webhook_code: String, item_id: String, environment: crate::models::WebhookEnvironmentValues) -> ProductPermissionsRequiredIdentityWebhook {
        ProductPermissionsRequiredIdentityWebhook {
            webhook_type,
            webhook_code,
            item_id,
            environment,
        }
    }
}


