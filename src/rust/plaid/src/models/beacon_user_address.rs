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

/// BeaconUserAddress : Even if an address has been collected, some fields may be null depending on the region's addressing system. For example:   Addresses from the United Kingdom will not include a region   Addresses from Hong Kong will not include a postal code
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeaconUserAddress {
    /// The primary street portion of an address. If an address is provided, this field will always be filled. A string with at least one non-whitespace alphabetical character, with a max length of 80 characters.
    #[serde(rename = "street")]
    pub street: String,
    /// Extra street information, like an apartment or suite number. If provided, a string with at least one non-whitespace character, with a max length of 50 characters.
    #[serde(rename = "street2", deserialize_with = "Option::deserialize")]
    pub street2: Option<String>,
    /// City from the end user's address. A string with at least one non-whitespace alphabetical character, with a max length of 100 characters.\"
    #[serde(rename = "city")]
    pub city: String,
    /// An ISO 3166-2 subdivision code. Related terms would be \"state\", \"province\", \"prefecture\", \"zone\", \"subdivision\", etc.
    #[serde(rename = "region", deserialize_with = "Option::deserialize")]
    pub region: Option<String>,
    /// The postal code for the associated address. Between 2 and 10 alphanumeric characters. For US-based addresses this must be 5 numeric digits.
    #[serde(rename = "postal_code", deserialize_with = "Option::deserialize")]
    pub postal_code: Option<String>,
    /// Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    #[serde(rename = "country")]
    pub country: String,
}

impl BeaconUserAddress {
    /// Even if an address has been collected, some fields may be null depending on the region's addressing system. For example:   Addresses from the United Kingdom will not include a region   Addresses from Hong Kong will not include a postal code
    pub fn new(street: String, street2: Option<String>, city: String, region: Option<String>, postal_code: Option<String>, country: String) -> BeaconUserAddress {
        BeaconUserAddress {
            street,
            street2,
            city,
            region,
            postal_code,
            country,
        }
    }
}

