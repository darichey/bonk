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

/// BankTransferUser : The legal name and other information for the account holder.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BankTransferUser {
    /// The account holder’s full legal name. If the transfer `ach_class` is `ccd`, this should be the business name of the account holder.
    #[serde(rename = "legal_name")]
    pub legal_name: String,
    /// The account holder’s email.
    #[serde(rename = "email_address", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<Option<String>>,
    /// The account holder's routing number. This field is only used in response data. Do not provide this field when making requests.
    #[serde(rename = "routing_number", skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}

impl BankTransferUser {
    /// The legal name and other information for the account holder.
    pub fn new(legal_name: String) -> BankTransferUser {
        BankTransferUser {
            legal_name,
            email_address: None,
            routing_number: None,
        }
    }
}

