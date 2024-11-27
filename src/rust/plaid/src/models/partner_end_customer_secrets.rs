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

/// PartnerEndCustomerSecrets : The secrets for the newly created end customer.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartnerEndCustomerSecrets {
    /// The end customer's secret key for the Sandbox environment.
    #[serde(rename = "sandbox", skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<String>,
    /// The end customer's secret key for the Development environment. The Development environment has been removed.
    #[serde(rename = "development", skip_serializing_if = "Option::is_none")]
    pub development: Option<String>,
    /// The end customer's secret key for the Production environment. The end customer will be provided with a limited number of credits to test in the Production environment before full enablement.
    #[serde(rename = "production", skip_serializing_if = "Option::is_none")]
    pub production: Option<String>,
}

impl PartnerEndCustomerSecrets {
    /// The secrets for the newly created end customer.
    pub fn new() -> PartnerEndCustomerSecrets {
        PartnerEndCustomerSecrets {
            sandbox: None,
            development: None,
            production: None,
        }
    }
}

