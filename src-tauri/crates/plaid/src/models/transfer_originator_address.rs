/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferOriginatorAddress : The originator's address.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferOriginatorAddress {
    /// The full city name.
    #[serde(rename = "city")]
    pub city: String,
    /// The full street address.
    #[serde(rename = "street")]
    pub street: String,
    /// The two-letter code for the state or province (e.g., \"CA\").
    #[serde(rename = "region")]
    pub region: String,
    /// The postal code (e.g., \"94103\").
    #[serde(rename = "postal_code")]
    pub postal_code: String,
    /// ISO-3166-1 alpha-2 country code standard.
    #[serde(rename = "country_code")]
    pub country_code: String,
}

impl TransferOriginatorAddress {
    /// The originator's address.
    pub fn new(city: String, street: String, region: String, postal_code: String, country_code: String) -> TransferOriginatorAddress {
        TransferOriginatorAddress {
            city,
            street,
            region,
            postal_code,
            country_code,
        }
    }
}


