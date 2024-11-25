/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankStatementUploadAccountOwnerAddress : Address on the uploaded bank statement



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditBankStatementUploadAccountOwnerAddress {
    /// The full city name.
    #[serde(rename = "city", deserialize_with = "Option::deserialize")]
    pub city: Option<String>,
    /// The ISO 3166-1 alpha-2 country code.
    #[serde(rename = "country", deserialize_with = "Option::deserialize")]
    pub country: Option<String>,
    /// The postal code of the address.
    #[serde(rename = "postal_code", deserialize_with = "Option::deserialize")]
    pub postal_code: Option<String>,
    /// The region or state. Example: `\"NC\"`
    #[serde(rename = "region", deserialize_with = "Option::deserialize")]
    pub region: Option<String>,
    /// The full street address.
    #[serde(rename = "street", deserialize_with = "Option::deserialize")]
    pub street: Option<String>,
}

impl CreditBankStatementUploadAccountOwnerAddress {
    /// Address on the uploaded bank statement
    pub fn new(city: Option<String>, country: Option<String>, postal_code: Option<String>, region: Option<String>, street: Option<String>) -> CreditBankStatementUploadAccountOwnerAddress {
        CreditBankStatementUploadAccountOwnerAddress {
            city,
            country,
            postal_code,
            region,
            street,
        }
    }
}


