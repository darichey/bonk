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

/// ClientProvidedEnrichedTransaction : A client-provided transaction that Plaid has enriched.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientProvidedEnrichedTransaction {
    /// The unique ID for the transaction as provided by you in the request.
    #[serde(rename = "id")]
    pub id: String,
    /// A unique user id used to group transactions for a given user, as a unique identifier from your application. Personally identifiable information, such as an email address or phone number, should not be used in the client_user_id.
    #[serde(rename = "client_user_id", skip_serializing_if = "Option::is_none")]
    pub client_user_id: Option<String>,
    /// A unique account id used to group transactions for a given account, as a unique identifier from your application. Personally identifiable information, such as an email address or phone number, should not be used in the client_account_id.
    #[serde(rename = "client_account_id", skip_serializing_if = "Option::is_none")]
    pub client_account_id: Option<String>,
    /// The account type associated with the transaction. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    #[serde(rename = "account_type", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// The account subtype associated with the transaction. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    #[serde(rename = "account_subtype", skip_serializing_if = "Option::is_none")]
    pub account_subtype: Option<String>,
    /// The raw description of the transaction.
    #[serde(rename = "description")]
    pub description: String,
    /// The absolute value of the transaction (>= 0)
    #[serde(rename = "amount")]
    pub amount: f64,
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<models::EnrichTransactionDirection>,
    /// The ISO-4217 currency code of the transaction e.g. USD.
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
    #[serde(rename = "enrichments")]
    pub enrichments: models::Enrichments,
}

impl ClientProvidedEnrichedTransaction {
    /// A client-provided transaction that Plaid has enriched.
    pub fn new(id: String, description: String, amount: f64, iso_currency_code: String, enrichments: models::Enrichments) -> ClientProvidedEnrichedTransaction {
        ClientProvidedEnrichedTransaction {
            id,
            client_user_id: None,
            client_account_id: None,
            account_type: None,
            account_subtype: None,
            description,
            amount,
            direction: None,
            iso_currency_code,
            enrichments,
        }
    }
}

