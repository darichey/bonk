/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RiskCheckBehaviorUserInteractionsLabel : Field describing the overall user interaction signals of a behavior risk check. This value represents how familiar the user is with the personal data they provide, based on a number of signals that are collected during their session.  `genuine` indicates the user has high familiarity with the data they are providing, and that fraud is unlikely.  `neutral` indicates some signals are present in between `risky` and `genuine`, but there are not enough clear signals to determine an outcome.  `risky` indicates the user has low familiarity with the data they are providing, and that fraud is likely.  `no_data` indicates there is not sufficient information to give an accurate signal.

/// Field describing the overall user interaction signals of a behavior risk check. This value represents how familiar the user is with the personal data they provide, based on a number of signals that are collected during their session.  `genuine` indicates the user has high familiarity with the data they are providing, and that fraud is unlikely.  `neutral` indicates some signals are present in between `risky` and `genuine`, but there are not enough clear signals to determine an outcome.  `risky` indicates the user has low familiarity with the data they are providing, and that fraud is likely.  `no_data` indicates there is not sufficient information to give an accurate signal.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RiskCheckBehaviorUserInteractionsLabel {
    #[serde(rename = "genuine")]
    Genuine,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "risky")]
    Risky,
    #[serde(rename = "no_data")]
    NoData,

}

impl ToString for RiskCheckBehaviorUserInteractionsLabel {
    fn to_string(&self) -> String {
        match self {
            Self::Genuine => String::from("genuine"),
            Self::Neutral => String::from("neutral"),
            Self::Risky => String::from("risky"),
            Self::NoData => String::from("no_data"),
        }
    }
}

impl Default for RiskCheckBehaviorUserInteractionsLabel {
    fn default() -> RiskCheckBehaviorUserInteractionsLabel {
        Self::Genuine
    }
}




