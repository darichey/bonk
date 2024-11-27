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

/// TaxpayerIdentifierType : A value from a MISMO prescribed list that classifies identification numbers used by the Internal Revenue Service (IRS) in the administration of tax laws. A Social Security number (SSN) is issued by the SSA; all other taxpayer identification numbers are issued by the IRS.
/// A value from a MISMO prescribed list that classifies identification numbers used by the Internal Revenue Service (IRS) in the administration of tax laws. A Social Security number (SSN) is issued by the SSA; all other taxpayer identification numbers are issued by the IRS.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaxpayerIdentifierType {
    #[serde(rename = "IndividualTaxpayerIdentificationNumber")]
    IndividualTaxpayerIdentificationNumber,
    #[serde(rename = "SocialSecurityNumber")]
    SocialSecurityNumber,

}

impl std::fmt::Display for TaxpayerIdentifierType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IndividualTaxpayerIdentificationNumber => write!(f, "IndividualTaxpayerIdentificationNumber"),
            Self::SocialSecurityNumber => write!(f, "SocialSecurityNumber"),
        }
    }
}

impl Default for TaxpayerIdentifierType {
    fn default() -> TaxpayerIdentifierType {
        Self::IndividualTaxpayerIdentificationNumber
    }
}

