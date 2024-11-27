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

/// AccountsGetResponse : AccountsGetResponse defines the response schema for `/accounts/get` and `/accounts/balance/get`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountsGetResponse {
    /// An array of financial institution accounts associated with the Item. If `/accounts/balance/get` was called, each account will include real-time balance information.
    #[serde(rename = "accounts")]
    pub accounts: Vec<models::AccountBase>,
    #[serde(rename = "item")]
    pub item: models::Item,
    #[serde(rename = "payment_risk_assessment", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payment_risk_assessment: Option<Option<models::AccountsBalanceGetResponsePaymentRiskAssessment>>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl AccountsGetResponse {
    /// AccountsGetResponse defines the response schema for `/accounts/get` and `/accounts/balance/get`.
    pub fn new(accounts: Vec<models::AccountBase>, item: models::Item, request_id: String) -> AccountsGetResponse {
        AccountsGetResponse {
            accounts,
            item,
            payment_risk_assessment: None,
            request_id,
        }
    }
}

