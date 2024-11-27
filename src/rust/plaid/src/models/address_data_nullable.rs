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

/// AddressDataNullable : Data about the components comprising an address.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressDataNullable {
    /// The full city name
    #[serde(rename = "city", deserialize_with = "Option::deserialize")]
    pub city: Option<String>,
    /// The region or state. In API versions 2018-05-22 and earlier, this field is called `state`. Example: `\"NC\"`
    #[serde(rename = "region", deserialize_with = "Option::deserialize")]
    pub region: Option<String>,
    /// The full street address Example: `\"564 Main Street, APT 15\"`
    #[serde(rename = "street")]
    pub street: String,
    /// The postal code. In API versions 2018-05-22 and earlier, this field is called `zip`.
    #[serde(rename = "postal_code", deserialize_with = "Option::deserialize")]
    pub postal_code: Option<String>,
    /// The ISO 3166-1 alpha-2 country code
    #[serde(rename = "country", deserialize_with = "Option::deserialize")]
    pub country: Option<String>,
}

impl AddressDataNullable {
    /// Data about the components comprising an address.
    pub fn new(city: Option<String>, region: Option<String>, street: String, postal_code: Option<String>, country: Option<String>) -> AddressDataNullable {
        AddressDataNullable {
            city,
            region,
            street,
            postal_code,
            country,
        }
    }
}

