/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Institution : Details relating to a specific financial institution



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Institution {
    /// Unique identifier for the institution. Note that the same institution may have multiple records, each with different institution IDs; for example, if the institution has migrated to OAuth, there may be separate `institution_id`s for the OAuth and non-OAuth versions of the institution. Institutions that operate in different countries or with multiple login portals may also have separate `institution_id`s for each country or portal.
    #[serde(rename = "institution_id")]
    pub institution_id: String,
    /// The official name of the institution.
    #[serde(rename = "name")]
    pub name: String,
    /// A list of the Plaid products supported by the institution. Note that only institutions that support Instant Auth will return `auth` in the product array; institutions that do not list `auth` may still support other Auth methods such as Instant Match or Automated Micro-deposit Verification. To identify institutions that support those methods, use the `auth_metadata` object. For more details, see [Full Auth coverage](https://plaid.com/docs/auth/coverage/).
    #[serde(rename = "products")]
    pub products: Vec<crate::models::Products>,
    /// A list of the country codes supported by the institution.
    #[serde(rename = "country_codes")]
    pub country_codes: Vec<crate::models::CountryCode>,
    /// The URL for the institution's website
    #[serde(rename = "url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub url: Option<Option<String>>,
    /// Hexadecimal representation of the primary color used by the institution
    #[serde(rename = "primary_color", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<Option<String>>,
    /// Base64 encoded representation of the institution's logo, returned as a base64 encoded 152x152 PNG. Not all institutions' logos are available.
    #[serde(rename = "logo", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub logo: Option<Option<String>>,
    /// A partial list of routing numbers associated with the institution. This list is provided for the purpose of looking up institutions by routing number. It is not comprehensive and should never be used as a complete list of routing numbers for an institution.
    #[serde(rename = "routing_numbers")]
    pub routing_numbers: Vec<String>,
    /// A partial list of DTC numbers associated with the institution.
    #[serde(rename = "dtc_numbers", skip_serializing_if = "Option::is_none")]
    pub dtc_numbers: Option<Vec<String>>,
    /// Indicates that the institution has an OAuth login flow. This will be `true` if OAuth is supported for any Items associated with the institution, even if the institution also supports non-OAuth connections.
    #[serde(rename = "oauth")]
    pub oauth: bool,
    #[serde(rename = "status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<crate::models::InstitutionStatus>>,
    #[serde(rename = "payment_initiation_metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payment_initiation_metadata: Option<Option<crate::models::PaymentInitiationMetadata>>,
    #[serde(rename = "auth_metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub auth_metadata: Option<Option<crate::models::AuthMetadata>>,
}

impl Institution {
    /// Details relating to a specific financial institution
    pub fn new(institution_id: String, name: String, products: Vec<crate::models::Products>, country_codes: Vec<crate::models::CountryCode>, routing_numbers: Vec<String>, oauth: bool) -> Institution {
        Institution {
            institution_id,
            name,
            products,
            country_codes,
            url: None,
            primary_color: None,
            logo: None,
            routing_numbers,
            dtc_numbers: None,
            oauth,
            status: None,
            payment_initiation_metadata: None,
            auth_metadata: None,
        }
    }
}


