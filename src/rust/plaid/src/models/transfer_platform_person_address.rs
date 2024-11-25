/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferPlatformPersonAddress : Home address of a person



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferPlatformPersonAddress {
    /// The full city name.
    #[serde(rename = "city")]
    pub city: String,
    /// Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    #[serde(rename = "country")]
    pub country: String,
    /// The postal code of the address.
    #[serde(rename = "postal_code")]
    pub postal_code: String,
    /// An ISO 3166-2 subdivision code. Related terms would be \"state\", \"province\", \"prefecture\", \"zone\", \"subdivision\", etc.
    #[serde(rename = "region")]
    pub region: String,
    /// The primary street portion of an address. A string with at least one non-whitespace alphabetical character, with a max length of 80 characters.
    #[serde(rename = "street")]
    pub street: String,
    /// Extra street information, like an apartment or suite number. If provided, a string with at least one non-whitespace character, with a max length of 50 characters.
    #[serde(rename = "street2", skip_serializing_if = "Option::is_none")]
    pub street2: Option<String>,
}

impl TransferPlatformPersonAddress {
    /// Home address of a person
    pub fn new(city: String, country: String, postal_code: String, region: String, street: String) -> TransferPlatformPersonAddress {
        TransferPlatformPersonAddress {
            city,
            country,
            postal_code,
            region,
            street,
            street2: None,
        }
    }
}


