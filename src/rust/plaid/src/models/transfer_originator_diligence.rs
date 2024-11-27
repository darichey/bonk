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

/// TransferOriginatorDiligence : The diligence information for the originator.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferOriginatorDiligence {
    /// The business name of the originator.
    #[serde(rename = "dba")]
    pub dba: String,
    /// The tax ID of the originator.
    #[serde(rename = "tax_id")]
    pub tax_id: String,
    #[serde(rename = "credit_usage_configuration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub credit_usage_configuration: Option<Option<Box<models::TransferCreditUsageConfiguration>>>,
    #[serde(rename = "debit_usage_configuration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub debit_usage_configuration: Option<Option<Box<models::TransferDebitUsageConfiguration>>>,
    #[serde(rename = "address")]
    pub address: Box<models::TransferOriginatorAddress>,
    /// The website of the originator.
    #[serde(rename = "website")]
    pub website: String,
    /// The NAICS code of the originator.
    #[serde(rename = "naics_code")]
    pub naics_code: String,
    #[serde(rename = "funding_account")]
    pub funding_account: Box<models::TransferFundingAccount>,
}

impl TransferOriginatorDiligence {
    /// The diligence information for the originator.
    pub fn new(dba: String, tax_id: String, address: models::TransferOriginatorAddress, website: String, naics_code: String, funding_account: models::TransferFundingAccount) -> TransferOriginatorDiligence {
        TransferOriginatorDiligence {
            dba,
            tax_id,
            credit_usage_configuration: None,
            debit_usage_configuration: None,
            address: Box::new(address),
            website,
            naics_code,
            funding_account: Box::new(funding_account),
        }
    }
}

