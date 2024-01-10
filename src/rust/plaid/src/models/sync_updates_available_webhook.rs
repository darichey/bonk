/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SyncUpdatesAvailableWebhook : Fired when an Item's transactions change. This can be due to any event resulting in new changes, such as an initial 30-day transactions fetch upon the initialization of an Item with transactions, the backfill of historical transactions that occurs shortly after, or when changes are populated from a regularly-scheduled transactions update job. It is recommended to listen for the `SYNC_UPDATES_AVAILABLE` webhook when using the `/transactions/sync` endpoint. Note that when using `/transactions/sync` the older webhooks `INITIAL_UPDATE`, `HISTORICAL_UPDATE`, `DEFAULT_UPDATE`, and `TRANSACTIONS_REMOVED`, which are intended for use with `/transactions/get`, will also continue to be sent in order to maintain backwards compatibility. It is not necessary to listen for and respond to those webhooks when using `/transactions/sync`.  After receipt of this webhook, the new changes can be fetched for the Item from `/transactions/sync`.  Note that to receive this webhook for an Item, `/transactions/sync` must have been called at least once on that Item. This means that, unlike the `INITIAL_UPDATE` and `HISTORICAL_UPDATE` webhooks, it will not fire immediately upon Item creation. If `/transactions/sync` is called on an Item that was *not* initialized with Transactions, the webhook will fire twice: once the first 30 days of transactions data has been fetched, and a second time when all available historical transactions data has been fetched.  This webhook will fire in the Sandbox environment as it would in Production. It can also be manually triggered in Sandbox by calling `/sandbox/item/fire_webhook`.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncUpdatesAvailableWebhook {
    /// `TRANSACTIONS`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `SYNC_UPDATES_AVAILABLE`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// Indicates if initial pull information (most recent 30 days of transaction history) is available.
    #[serde(rename = "initial_update_complete")]
    pub initial_update_complete: bool,
    /// Indicates if historical pull information (maximum transaction history requested, up to 2 years) is available.
    #[serde(rename = "historical_update_complete")]
    pub historical_update_complete: bool,
    #[serde(rename = "environment")]
    pub environment: crate::models::WebhookEnvironmentValues,
}

impl SyncUpdatesAvailableWebhook {
    /// Fired when an Item's transactions change. This can be due to any event resulting in new changes, such as an initial 30-day transactions fetch upon the initialization of an Item with transactions, the backfill of historical transactions that occurs shortly after, or when changes are populated from a regularly-scheduled transactions update job. It is recommended to listen for the `SYNC_UPDATES_AVAILABLE` webhook when using the `/transactions/sync` endpoint. Note that when using `/transactions/sync` the older webhooks `INITIAL_UPDATE`, `HISTORICAL_UPDATE`, `DEFAULT_UPDATE`, and `TRANSACTIONS_REMOVED`, which are intended for use with `/transactions/get`, will also continue to be sent in order to maintain backwards compatibility. It is not necessary to listen for and respond to those webhooks when using `/transactions/sync`.  After receipt of this webhook, the new changes can be fetched for the Item from `/transactions/sync`.  Note that to receive this webhook for an Item, `/transactions/sync` must have been called at least once on that Item. This means that, unlike the `INITIAL_UPDATE` and `HISTORICAL_UPDATE` webhooks, it will not fire immediately upon Item creation. If `/transactions/sync` is called on an Item that was *not* initialized with Transactions, the webhook will fire twice: once the first 30 days of transactions data has been fetched, and a second time when all available historical transactions data has been fetched.  This webhook will fire in the Sandbox environment as it would in Production. It can also be manually triggered in Sandbox by calling `/sandbox/item/fire_webhook`.
    pub fn new(webhook_type: String, webhook_code: String, item_id: String, initial_update_complete: bool, historical_update_complete: bool, environment: crate::models::WebhookEnvironmentValues) -> SyncUpdatesAvailableWebhook {
        SyncUpdatesAvailableWebhook {
            webhook_type,
            webhook_code,
            item_id,
            initial_update_complete,
            historical_update_complete,
            environment,
        }
    }
}

