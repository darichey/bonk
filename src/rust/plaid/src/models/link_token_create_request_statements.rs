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

/// LinkTokenCreateRequestStatements : Specifies options for initializing Link for use with the Statements product. This field is required for the statements product.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestStatements {
    /// The start date for statements, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) \"YYYY-MM-DD\" format, e.g. \"2020-10-30\".
    #[serde(rename = "start_date")]
    pub start_date: String,
    /// The end date for statements, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) \"YYYY-MM-DD\" format, e.g. \"2020-10-30\". You can request up to two years of data.
    #[serde(rename = "end_date")]
    pub end_date: String,
}

impl LinkTokenCreateRequestStatements {
    /// Specifies options for initializing Link for use with the Statements product. This field is required for the statements product.
    pub fn new(start_date: String, end_date: String) -> LinkTokenCreateRequestStatements {
        LinkTokenCreateRequestStatements {
            start_date,
            end_date,
        }
    }
}

