/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferIntentCreateRequest : Defines the request schema for `/transfer/intent/create`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferIntentCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The Plaid `account_id` corresponding to the end-user account that will be debited or credited.
    #[serde(rename = "account_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Option<String>>,
    /// Specify the account used to fund the transfer. Should be specified if using legacy funding methods only. If using Plaid Ledger, leave this field blank. Customers can find a list of `funding_account_id`s in the Accounts page of your Plaid Dashboard, under the \"Account ID\" column. If this field is left blank and you are using legacy funding methods, this will default to the default `funding_account_id` specified during onboarding. Otherwise, Plaid Ledger will be used.
    #[serde(rename = "funding_account_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub funding_account_id: Option<Option<String>>,
    #[serde(rename = "mode")]
    pub mode: crate::models::TransferIntentCreateMode,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<crate::models::TransferIntentCreateNetwork>,
    /// The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\"). When calling `/transfer/authorization/create`, specify the maximum amount to authorize. When calling `/transfer/create`, specify the exact amount of the transfer, up to a maximum of the amount authorized. If this field is left blank when calling `/transfer/create`, the maximum amount authorized in the `authorization_id` will be sent.
    #[serde(rename = "amount")]
    pub amount: String,
    /// A description for the underlying transfer. Maximum of 8 characters.
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "ach_class", skip_serializing_if = "Option::is_none")]
    pub ach_class: Option<crate::models::AchClass>,
    /// Plaid’s unique identifier for the origination account for the intent. If not provided, the default account will be used.
    #[serde(rename = "origination_account_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub origination_account_id: Option<Option<String>>,
    #[serde(rename = "user")]
    pub user: Box<crate::models::TransferUserInRequest>,
    /// The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply: The JSON values must be Strings (no nested JSON objects allowed) Only ASCII characters may be used Maximum of 50 key/value pairs Maximum key length of 40 characters Maximum value length of 500 characters 
    #[serde(rename = "metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Option<::std::collections::HashMap<String, String>>>,
    /// The currency of the transfer amount, e.g. \"USD\"
    #[serde(rename = "iso_currency_code", skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    /// When `true`, the transfer requires a `GUARANTEED` decision by Plaid to proceed (Guarantee customers only).
    #[serde(rename = "require_guarantee", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub require_guarantee: Option<Option<bool>>,
}

impl TransferIntentCreateRequest {
    /// Defines the request schema for `/transfer/intent/create`
    pub fn new(mode: crate::models::TransferIntentCreateMode, amount: String, description: String, user: crate::models::TransferUserInRequest) -> TransferIntentCreateRequest {
        TransferIntentCreateRequest {
            client_id: None,
            secret: None,
            account_id: None,
            funding_account_id: None,
            mode,
            network: None,
            amount,
            description,
            ach_class: None,
            origination_account_id: None,
            user: Box::new(user),
            metadata: None,
            iso_currency_code: None,
            require_guarantee: None,
        }
    }
}

