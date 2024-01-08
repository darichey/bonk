/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BeaconReportType : The type of Beacon Report.  `first_party`: If this is the same individual as the one who submitted the KYC.  `stolen`: If this is a different individual from the one who submitted the KYC.  `synthetic`: If this is an individual using fabricated information.  `account_takeover`: If this individual's account was compromised.  `unknown`: If you aren't sure who committed the fraud.

/// The type of Beacon Report.  `first_party`: If this is the same individual as the one who submitted the KYC.  `stolen`: If this is a different individual from the one who submitted the KYC.  `synthetic`: If this is an individual using fabricated information.  `account_takeover`: If this individual's account was compromised.  `unknown`: If you aren't sure who committed the fraud.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BeaconReportType {
    #[serde(rename = "first_party")]
    FirstParty,
    #[serde(rename = "stolen")]
    Stolen,
    #[serde(rename = "synthetic")]
    Synthetic,
    #[serde(rename = "account_takeover")]
    AccountTakeover,
    #[serde(rename = "unknown")]
    Unknown,

}

impl ToString for BeaconReportType {
    fn to_string(&self) -> String {
        match self {
            Self::FirstParty => String::from("first_party"),
            Self::Stolen => String::from("stolen"),
            Self::Synthetic => String::from("synthetic"),
            Self::AccountTakeover => String::from("account_takeover"),
            Self::Unknown => String::from("unknown"),
        }
    }
}

impl Default for BeaconReportType {
    fn default() -> BeaconReportType {
        Self::FirstParty
    }
}




