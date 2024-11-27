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

/// EntityDocumentType : The kind of official document represented by this object.  `bik` - Russian bank code  `business_number` - A number that uniquely identifies the business within a category of businesses  `imo` - Number assigned to the entity by the International Maritime Organization  `other` - Any document not covered by other categories  `swift` - Number identifying a bank and branch.  `tax_id` - Identification issued for the purpose of collecting taxes
/// The kind of official document represented by this object.  `bik` - Russian bank code  `business_number` - A number that uniquely identifies the business within a category of businesses  `imo` - Number assigned to the entity by the International Maritime Organization  `other` - Any document not covered by other categories  `swift` - Number identifying a bank and branch.  `tax_id` - Identification issued for the purpose of collecting taxes
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EntityDocumentType {
    #[serde(rename = "bik")]
    Bik,
    #[serde(rename = "business_number")]
    BusinessNumber,
    #[serde(rename = "imo")]
    Imo,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "swift")]
    Swift,
    #[serde(rename = "tax_id")]
    TaxId,

}

impl std::fmt::Display for EntityDocumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Bik => write!(f, "bik"),
            Self::BusinessNumber => write!(f, "business_number"),
            Self::Imo => write!(f, "imo"),
            Self::Other => write!(f, "other"),
            Self::Swift => write!(f, "swift"),
            Self::TaxId => write!(f, "tax_id"),
        }
    }
}

impl Default for EntityDocumentType {
    fn default() -> EntityDocumentType {
        Self::Bik
    }
}

