/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RiskCheckBehaviorBotDetectedLabel : Field describing the outcome of a bot detection behavior risk check.  `yes` indicates that automated activity was detected.  `no` indicates that automated activity was not detected.  `no_data` indicates there was not enough information available to give an accurate signal.

/// Field describing the outcome of a bot detection behavior risk check.  `yes` indicates that automated activity was detected.  `no` indicates that automated activity was not detected.  `no_data` indicates there was not enough information available to give an accurate signal.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RiskCheckBehaviorBotDetectedLabel {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "no_data")]
    NoData,

}

impl ToString for RiskCheckBehaviorBotDetectedLabel {
    fn to_string(&self) -> String {
        match self {
            Self::Yes => String::from("yes"),
            Self::No => String::from("no"),
            Self::NoData => String::from("no_data"),
        }
    }
}

impl Default for RiskCheckBehaviorBotDetectedLabel {
    fn default() -> RiskCheckBehaviorBotDetectedLabel {
        Self::Yes
    }
}




