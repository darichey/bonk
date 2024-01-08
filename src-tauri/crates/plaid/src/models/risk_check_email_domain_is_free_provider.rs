/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RiskCheckEmailDomainIsFreeProvider : Indicates whether the email address domain is a free provider such as Gmail or Hotmail if known.

/// Indicates whether the email address domain is a free provider such as Gmail or Hotmail if known.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RiskCheckEmailDomainIsFreeProvider {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "no_data")]
    NoData,

}

impl ToString for RiskCheckEmailDomainIsFreeProvider {
    fn to_string(&self) -> String {
        match self {
            Self::Yes => String::from("yes"),
            Self::No => String::from("no"),
            Self::NoData => String::from("no_data"),
        }
    }
}

impl Default for RiskCheckEmailDomainIsFreeProvider {
    fn default() -> RiskCheckEmailDomainIsFreeProvider {
        Self::Yes
    }
}




