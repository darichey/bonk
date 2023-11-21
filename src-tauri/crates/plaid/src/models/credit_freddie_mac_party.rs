/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditFreddieMacParty : A collection of information about a single party to a transaction. Included direct participants like the borrower and seller as well as indirect participants such as the flood certificate provider.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditFreddieMacParty {
    #[serde(rename = "INDIVIDUAL")]
    pub individual: crate::models::CreditFreddieMacPartyIndividual,
    #[serde(rename = "ROLES")]
    pub roles: crate::models::Roles,
    #[serde(rename = "TAXPAYER_IDENTIFIERS")]
    pub taxpayer_identifiers: crate::models::TaxpayerIdentifiers,
}

impl CreditFreddieMacParty {
    /// A collection of information about a single party to a transaction. Included direct participants like the borrower and seller as well as indirect participants such as the flood certificate provider.
    pub fn new(individual: crate::models::CreditFreddieMacPartyIndividual, roles: crate::models::Roles, taxpayer_identifiers: crate::models::TaxpayerIdentifiers) -> CreditFreddieMacParty {
        CreditFreddieMacParty {
            individual,
            roles,
            taxpayer_identifiers,
        }
    }
}

