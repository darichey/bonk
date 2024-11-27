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

/// CreditFreddieMacVerificationOfAssetsDeal : An object representing an Asset Report with Freddie Mac schema.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditFreddieMacVerificationOfAssetsDeal {
    #[serde(rename = "LOANS")]
    pub loans: models::CreditFreddieMacLoans,
    #[serde(rename = "PARTIES")]
    pub parties: models::CreditFreddieMacParties,
    #[serde(rename = "SERVICES")]
    pub services: models::CreditFreddieMacServices,
}

impl CreditFreddieMacVerificationOfAssetsDeal {
    /// An object representing an Asset Report with Freddie Mac schema.
    pub fn new(loans: models::CreditFreddieMacLoans, parties: models::CreditFreddieMacParties, services: models::CreditFreddieMacServices) -> CreditFreddieMacVerificationOfAssetsDeal {
        CreditFreddieMacVerificationOfAssetsDeal {
            loans,
            parties,
            services,
        }
    }
}

