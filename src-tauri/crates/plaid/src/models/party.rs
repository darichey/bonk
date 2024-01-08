/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Party : A collection of information about a single party to a transaction. Included direct participants like the borrower and seller as well as indirect participants such as the flood certificate provider.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Party {
    #[serde(rename = "INDIVIDUAL")]
    pub individual: crate::models::PartyIndividual,
    #[serde(rename = "ROLES")]
    pub roles: crate::models::Roles,
    #[serde(rename = "TAXPAYER_IDENTIFIERS")]
    pub taxpayer_identifiers: crate::models::TaxpayerIdentifiers,
}

impl Party {
    /// A collection of information about a single party to a transaction. Included direct participants like the borrower and seller as well as indirect participants such as the flood certificate provider.
    pub fn new(individual: crate::models::PartyIndividual, roles: crate::models::Roles, taxpayer_identifiers: crate::models::TaxpayerIdentifiers) -> Party {
        Party {
            individual,
            roles,
            taxpayer_identifiers,
        }
    }
}


