/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferAuthorizationProposedTransfer : Details regarding the proposed transfer.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferAuthorizationProposedTransfer {
    #[serde(rename = "ach_class", skip_serializing_if = "Option::is_none")]
    pub ach_class: Option<crate::models::AchClass>,
    /// The Plaid `account_id` for the account that will be debited or credited.
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The id of the associated funding account, available in the Plaid Dashboard. If present, this indicates which of your business checking accounts will be credited or debited.
    #[serde(rename = "funding_account_id", deserialize_with = "Option::deserialize")]
    pub funding_account_id: Option<String>,
    /// Plaid’s unique identifier for a Plaid Ledger Balance.
    #[serde(rename = "ledger_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ledger_id: Option<Option<String>>,
    #[serde(rename = "type")]
    pub r#type: crate::models::TransferType,
    #[serde(rename = "user")]
    pub user: crate::models::TransferUserInResponse,
    /// The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\"). When calling `/transfer/authorization/create`, specify the maximum amount to authorize. When calling `/transfer/create`, specify the exact amount of the transfer, up to a maximum of the amount authorized. If this field is left blank when calling `/transfer/create`, the maximum amount authorized in the `authorization_id` will be sent.
    #[serde(rename = "amount")]
    pub amount: String,
    /// The network or rails used for the transfer.
    #[serde(rename = "network")]
    pub network: String,
    #[serde(rename = "wire_details", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub wire_details: Option<Option<Box<crate::models::TransferWireDetails>>>,
    /// Plaid's unique identifier for the origination account that was used for this transfer.
    #[serde(rename = "origination_account_id")]
    pub origination_account_id: String,
    /// The currency of the transfer amount. The default value is \"USD\".
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
    /// The Plaid client ID that is the originator of this transfer. Only present if created on behalf of another client as a [Platform customer](https://plaid.com/docs/transfer/application/#originators-vs-platforms).
    #[serde(rename = "originator_client_id", deserialize_with = "Option::deserialize")]
    pub originator_client_id: Option<String>,
    #[serde(rename = "credit_funds_source", deserialize_with = "Option::deserialize")]
    pub credit_funds_source: Option<crate::models::TransferCreditFundsSource>,
}

impl TransferAuthorizationProposedTransfer {
    /// Details regarding the proposed transfer.
    pub fn new(funding_account_id: Option<String>, r#type: crate::models::TransferType, user: crate::models::TransferUserInResponse, amount: String, network: String, origination_account_id: String, iso_currency_code: String, originator_client_id: Option<String>, credit_funds_source: Option<crate::models::TransferCreditFundsSource>) -> TransferAuthorizationProposedTransfer {
        TransferAuthorizationProposedTransfer {
            ach_class: None,
            account_id: None,
            funding_account_id,
            ledger_id: None,
            r#type,
            user,
            amount,
            network,
            wire_details: None,
            origination_account_id,
            iso_currency_code,
            originator_client_id,
            credit_funds_source,
        }
    }
}


