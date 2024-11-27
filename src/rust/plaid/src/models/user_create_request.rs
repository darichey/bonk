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

/// UserCreateRequest : UserCreateRequest defines the request schema for `/user/create`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// A unique ID representing the end user. Maximum of 128 characters. Typically this will be a user ID number from your application. Personally identifiable information, such as an email address or phone number, should not be used in the `client_user_id`.
    #[serde(rename = "client_user_id")]
    pub client_user_id: String,
    /// A unique ID representing a CRA reseller's end customer. Maximum of 128 characters.
    #[serde(rename = "end_customer", skip_serializing_if = "Option::is_none")]
    pub end_customer: Option<String>,
    #[serde(rename = "consumer_report_user_identity", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub consumer_report_user_identity: Option<Option<models::ConsumerReportUserIdentity>>,
}

impl UserCreateRequest {
    /// UserCreateRequest defines the request schema for `/user/create`
    pub fn new(client_user_id: String) -> UserCreateRequest {
        UserCreateRequest {
            client_id: None,
            secret: None,
            client_user_id,
            end_customer: None,
            consumer_report_user_identity: None,
        }
    }
}

