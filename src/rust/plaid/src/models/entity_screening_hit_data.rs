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

/// EntityScreeningHitData : Information associated with the entity watchlist hit
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityScreeningHitData {
    /// Documents associated with the watchlist hit
    #[serde(rename = "documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<models::EntityScreeningHitDocumentsItems>>,
    /// Email addresses associated with the watchlist hit
    #[serde(rename = "email_addresses", skip_serializing_if = "Option::is_none")]
    pub email_addresses: Option<Vec<models::EntityScreeningHitEmailsItems>>,
    /// Locations associated with the watchlist hit
    #[serde(rename = "locations", skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<models::GenericScreeningHitLocationItems>>,
    /// Names associated with the watchlist hit
    #[serde(rename = "names", skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<models::EntityScreeningHitNamesItems>>,
    /// Phone numbers associated with the watchlist hit
    #[serde(rename = "phone_numbers", skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<models::EntityScreeningHitsPhoneNumberItems>>,
    /// URLs associated with the watchlist hit
    #[serde(rename = "urls", skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<models::EntityScreeningHitUrlsItems>>,
}

impl EntityScreeningHitData {
    /// Information associated with the entity watchlist hit
    pub fn new() -> EntityScreeningHitData {
        EntityScreeningHitData {
            documents: None,
            email_addresses: None,
            locations: None,
            names: None,
            phone_numbers: None,
            urls: None,
        }
    }
}

