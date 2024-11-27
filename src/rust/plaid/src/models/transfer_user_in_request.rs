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

/// TransferUserInRequest : The legal name and other information for the account holder.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferUserInRequest {
    /// The user's legal name.
    #[serde(rename = "legal_name")]
    pub legal_name: String,
    /// The user's phone number.
    #[serde(rename = "phone_number", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// The user's email address.
    #[serde(rename = "email_address", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Box<models::TransferUserAddressInRequest>>,
}

impl TransferUserInRequest {
    /// The legal name and other information for the account holder.
    pub fn new(legal_name: String) -> TransferUserInRequest {
        TransferUserInRequest {
            legal_name,
            phone_number: None,
            email_address: None,
            address: None,
        }
    }
}

