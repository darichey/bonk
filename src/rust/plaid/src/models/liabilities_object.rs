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

/// LiabilitiesObject : An object containing liability accounts
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiabilitiesObject {
    /// The credit accounts returned.
    #[serde(rename = "credit", deserialize_with = "Option::deserialize")]
    pub credit: Option<Vec<models::CreditCardLiability>>,
    /// The mortgage accounts returned.
    #[serde(rename = "mortgage", deserialize_with = "Option::deserialize")]
    pub mortgage: Option<Vec<models::MortgageLiability>>,
    /// The student loan accounts returned.
    #[serde(rename = "student", deserialize_with = "Option::deserialize")]
    pub student: Option<Vec<models::StudentLoan>>,
}

impl LiabilitiesObject {
    /// An object containing liability accounts
    pub fn new(credit: Option<Vec<models::CreditCardLiability>>, mortgage: Option<Vec<models::MortgageLiability>>, student: Option<Vec<models::StudentLoan>>) -> LiabilitiesObject {
        LiabilitiesObject {
            credit,
            mortgage,
            student,
        }
    }
}

