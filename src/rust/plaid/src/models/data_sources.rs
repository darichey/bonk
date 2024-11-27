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

/// DataSources : A description of the source of data for a given product/data type.  `INSTITUTION`: The institution supports this product, and the data was provided by the institution. `INSTITUTION_MASK`: The user manually provided the full account number, which was matched to the account mask provided by the institution. Only applicable to the `numbers` data type. `USER`: The institution does not support this product, and the data was manually provided by the user.
/// A description of the source of data for a given product/data type.  `INSTITUTION`: The institution supports this product, and the data was provided by the institution. `INSTITUTION_MASK`: The user manually provided the full account number, which was matched to the account mask provided by the institution. Only applicable to the `numbers` data type. `USER`: The institution does not support this product, and the data was manually provided by the user.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DataSources {
    #[serde(rename = "INSTITUTION")]
    Institution,
    #[serde(rename = "INSTITUTION_MASK")]
    InstitutionMask,
    #[serde(rename = "USER")]
    User,

}

impl std::fmt::Display for DataSources {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Institution => write!(f, "INSTITUTION"),
            Self::InstitutionMask => write!(f, "INSTITUTION_MASK"),
            Self::User => write!(f, "USER"),
        }
    }
}

impl Default for DataSources {
    fn default() -> DataSources {
        Self::Institution
    }
}

