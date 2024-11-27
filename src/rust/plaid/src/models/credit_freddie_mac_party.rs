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

/// CreditFreddieMacParty : A collection of information about a single party to a transaction. Included direct participants like the borrower and seller as well as indirect participants such as the flood certificate provider.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditFreddieMacParty {
    #[serde(rename = "INDIVIDUAL")]
    pub individual: models::CreditFreddieMacPartyIndividual,
    #[serde(rename = "ROLES")]
    pub roles: models::Roles,
    #[serde(rename = "TAXPAYER_IDENTIFIERS")]
    pub taxpayer_identifiers: models::TaxpayerIdentifiers,
}

impl CreditFreddieMacParty {
    /// A collection of information about a single party to a transaction. Included direct participants like the borrower and seller as well as indirect participants such as the flood certificate provider.
    pub fn new(individual: models::CreditFreddieMacPartyIndividual, roles: models::Roles, taxpayer_identifiers: models::TaxpayerIdentifiers) -> CreditFreddieMacParty {
        CreditFreddieMacParty {
            individual,
            roles,
            taxpayer_identifiers,
        }
    }
}

