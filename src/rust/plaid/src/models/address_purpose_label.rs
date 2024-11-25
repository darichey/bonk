/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AddressPurposeLabel : Field describing whether the associated address is being used for commercial or residential purposes.  Note: This value will be `no_data` when Plaid does not have sufficient data to determine the address's use.

/// Field describing whether the associated address is being used for commercial or residential purposes.  Note: This value will be `no_data` when Plaid does not have sufficient data to determine the address's use.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AddressPurposeLabel {
    #[serde(rename = "residential")]
    Residential,
    #[serde(rename = "commercial")]
    Commercial,
    #[serde(rename = "no_data")]
    NoData,

}

impl ToString for AddressPurposeLabel {
    fn to_string(&self) -> String {
        match self {
            Self::Residential => String::from("residential"),
            Self::Commercial => String::from("commercial"),
            Self::NoData => String::from("no_data"),
        }
    }
}

impl Default for AddressPurposeLabel {
    fn default() -> AddressPurposeLabel {
        Self::Residential
    }
}




