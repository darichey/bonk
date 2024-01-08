/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RecurringTransferNullable : Represents a recurring transfer within the Transfers API.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecurringTransferNullable {
    /// Plaid’s unique identifier for a recurring transfer.
    #[serde(rename = "recurring_transfer_id")]
    pub recurring_transfer_id: String,
    /// The datetime when this transfer was created. This will be of the form `2006-01-02T15:04:05Z`
    #[serde(rename = "created")]
    pub created: String,
    /// A date in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).  The next transfer origination date after bank holiday adjustment.
    #[serde(rename = "next_origination_date", deserialize_with = "Option::deserialize")]
    pub next_origination_date: Option<String>,
    /// Plaid’s unique identifier for a test clock.
    #[serde(rename = "test_clock_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub test_clock_id: Option<Option<String>>,
    #[serde(rename = "type")]
    pub r#type: crate::models::TransferType,
    /// The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\"). When calling `/transfer/authorization/create`, specify the maximum amount to authorize. When calling `/transfer/create`, specify the exact amount of the transfer, up to a maximum of the amount authorized. If this field is left blank when calling `/transfer/create`, the maximum amount authorized in the `authorization_id` will be sent.
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "status")]
    pub status: crate::models::TransferRecurringStatus,
    #[serde(rename = "ach_class", skip_serializing_if = "Option::is_none")]
    pub ach_class: Option<crate::models::AchClass>,
    #[serde(rename = "network")]
    pub network: crate::models::TransferNetwork,
    /// Plaid’s unique identifier for the origination account that was used for this transfer.
    #[serde(rename = "origination_account_id")]
    pub origination_account_id: String,
    /// The Plaid `account_id` corresponding to the end-user account that will be debited or credited.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The id of the funding account to use, available in the Plaid Dashboard. This determines which of your business checking accounts will be credited or debited.
    #[serde(rename = "funding_account_id")]
    pub funding_account_id: String,
    /// The currency of the transfer amount, e.g. \"USD\"
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
    /// The description of the recurring transfer.
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "transfer_ids")]
    pub transfer_ids: Vec<String>,
    #[serde(rename = "user")]
    pub user: crate::models::TransferUserInResponse,
    #[serde(rename = "schedule")]
    pub schedule: Box<crate::models::TransferRecurringSchedule>,
}

impl RecurringTransferNullable {
    /// Represents a recurring transfer within the Transfers API.
    pub fn new(recurring_transfer_id: String, created: String, next_origination_date: Option<String>, r#type: crate::models::TransferType, amount: String, status: crate::models::TransferRecurringStatus, network: crate::models::TransferNetwork, origination_account_id: String, account_id: String, funding_account_id: String, iso_currency_code: String, description: String, transfer_ids: Vec<String>, user: crate::models::TransferUserInResponse, schedule: crate::models::TransferRecurringSchedule) -> RecurringTransferNullable {
        RecurringTransferNullable {
            recurring_transfer_id,
            created,
            next_origination_date,
            test_clock_id: None,
            r#type,
            amount,
            status,
            ach_class: None,
            network,
            origination_account_id,
            account_id,
            funding_account_id,
            iso_currency_code,
            description,
            transfer_ids,
            user,
            schedule: Box::new(schedule),
        }
    }
}


