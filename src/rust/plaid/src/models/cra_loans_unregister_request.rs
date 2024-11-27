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

/// CraLoansUnregisterRequest : CraLoansUnregisterRequest defines the request schema for `/cra/loans/unregister`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CraLoansUnregisterRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// A list of loans to unregister.
    #[serde(rename = "loans")]
    pub loans: Vec<models::CraLoanUnregister>,
}

impl CraLoansUnregisterRequest {
    /// CraLoansUnregisterRequest defines the request schema for `/cra/loans/unregister`
    pub fn new(loans: Vec<models::CraLoanUnregister>) -> CraLoansUnregisterRequest {
        CraLoansUnregisterRequest {
            client_id: None,
            secret: None,
            loans,
        }
    }
}

