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

/// LinkTokenCreateRequestCreditPartnerInsights : Specifies options for initializing Link for use with the Credit Partner Insights product.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestCreditPartnerInsights {
    /// The maximum integer number of days of history to compute Credit Partner Insights. Defaults to 180 if not specified
    #[serde(rename = "days_requested", skip_serializing_if = "Option::is_none")]
    pub days_requested: Option<i32>,
    /// The specific Prism products to return. If none are passed in, then all products will be returned.
    #[serde(rename = "prism_products", skip_serializing_if = "Option::is_none")]
    pub prism_products: Option<Vec<models::PrismProduct>>,
}

impl LinkTokenCreateRequestCreditPartnerInsights {
    /// Specifies options for initializing Link for use with the Credit Partner Insights product.
    pub fn new() -> LinkTokenCreateRequestCreditPartnerInsights {
        LinkTokenCreateRequestCreditPartnerInsights {
            days_requested: None,
            prism_products: None,
        }
    }
}

