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

/// CraBankIncomeCreateResponse : CraBankIncomeCreateRequest defines the response schema for `/cra/bank_income/create`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CraBankIncomeCreateResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl CraBankIncomeCreateResponse {
    /// CraBankIncomeCreateRequest defines the response schema for `/cra/bank_income/create`.
    pub fn new() -> CraBankIncomeCreateResponse {
        CraBankIncomeCreateResponse {
            request_id: None,
        }
    }
}

