/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

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

impl ToString for EntityDocumentType {
    fn to_string(&self) -> String {
        match self {
            Self::Bik => String::from("bik"),
            Self::BusinessNumber => String::from("business_number"),
            Self::Imo => String::from("imo"),
            Self::Other => String::from("other"),
            Self::Swift => String::from("swift"),
            Self::TaxId => String::from("tax_id"),
        }
    }
}

impl Default for EntityDocumentType {
    fn default() -> EntityDocumentType {
        Self::Bik
    }
}




