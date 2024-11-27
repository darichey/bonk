/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// TransferIntentCreate : Represents a transfer intent within Transfer UI.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferIntentCreate {
    /// Plaid's unique identifier for the transfer intent object.
    #[serde(rename = "id")]
    pub id: String,
    /// The datetime the transfer was created. This will be of the form `2006-01-02T15:04:05Z`.
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "status")]
    pub status: models::TransferIntentStatus,
    /// The Plaid `account_id` corresponding to the end-user account that will be debited or credited. Returned only if `account_id` was set on intent creation.
    #[serde(rename = "account_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Option<String>>,
    /// Plaid’s unique identifier for the origination account for the intent. If not provided, the default account will be used.
    #[serde(rename = "origination_account_id")]
    pub origination_account_id: String,
    /// The id of the funding account to use, available in the Plaid Dashboard. This determines which of your business checking accounts will be credited or debited.
    #[serde(rename = "funding_account_id")]
    pub funding_account_id: String,
    /// The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\"). When calling `/transfer/authorization/create`, specify the maximum amount to authorize. When calling `/transfer/create`, specify the exact amount of the transfer, up to a maximum of the amount authorized. If this field is left blank when calling `/transfer/create`, the maximum amount authorized in the `authorization_id` will be sent.
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "mode")]
    pub mode: models::TransferIntentCreateMode,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<models::TransferIntentCreateNetwork>,
    #[serde(rename = "ach_class", skip_serializing_if = "Option::is_none")]
    pub ach_class: Option<models::AchClass>,
    #[serde(rename = "user")]
    pub user: models::TransferUserInResponse,
    /// A description for the underlying transfer. Maximum of 8 characters.
    #[serde(rename = "description")]
    pub description: String,
    /// The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply: The JSON values must be Strings (no nested JSON objects allowed) Only ASCII characters may be used Maximum of 50 key/value pairs Maximum key length of 40 characters Maximum value length of 500 characters 
    #[serde(rename = "metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Option<std::collections::HashMap<String, String>>>,
    /// The currency of the transfer amount, e.g. \"USD\"
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
    /// When `true`, the transfer requires a `GUARANTEED` decision by Plaid to proceed (Guarantee customers only).
    #[serde(rename = "require_guarantee", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub require_guarantee: Option<Option<bool>>,
}

impl TransferIntentCreate {
    /// Represents a transfer intent within Transfer UI.
    pub fn new(id: String, created: String, status: models::TransferIntentStatus, origination_account_id: String, funding_account_id: String, amount: String, mode: models::TransferIntentCreateMode, user: models::TransferUserInResponse, description: String, iso_currency_code: String) -> TransferIntentCreate {
        TransferIntentCreate {
            id,
            created,
            status,
            account_id: None,
            origination_account_id,
            funding_account_id,
            amount,
            mode,
            network: None,
            ach_class: None,
            user,
            description,
            metadata: None,
            iso_currency_code,
            require_guarantee: None,
        }
    }
}

