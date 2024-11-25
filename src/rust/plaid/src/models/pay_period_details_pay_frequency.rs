/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PayPeriodDetailsPayFrequency : The frequency at which an individual is paid.

/// The frequency at which an individual is paid.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PayPeriodDetailsPayFrequency {
    #[serde(rename = "PAY_FREQUENCY_UNKNOWN")]
    PayFrequencyUnknown,
    #[serde(rename = "PAY_FREQUENCY_WEEKLY")]
    PayFrequencyWeekly,
    #[serde(rename = "PAY_FREQUENCY_BIWEEKLY")]
    PayFrequencyBiweekly,
    #[serde(rename = "PAY_FREQUENCY_SEMIMONTHLY")]
    PayFrequencySemimonthly,
    #[serde(rename = "PAY_FREQUENCY_MONTHLY")]
    PayFrequencyMonthly,
    #[serde(rename = "null")]
    Null,

}

impl ToString for PayPeriodDetailsPayFrequency {
    fn to_string(&self) -> String {
        match self {
            Self::PayFrequencyUnknown => String::from("PAY_FREQUENCY_UNKNOWN"),
            Self::PayFrequencyWeekly => String::from("PAY_FREQUENCY_WEEKLY"),
            Self::PayFrequencyBiweekly => String::from("PAY_FREQUENCY_BIWEEKLY"),
            Self::PayFrequencySemimonthly => String::from("PAY_FREQUENCY_SEMIMONTHLY"),
            Self::PayFrequencyMonthly => String::from("PAY_FREQUENCY_MONTHLY"),
            Self::Null => String::from("null"),
        }
    }
}

impl Default for PayPeriodDetailsPayFrequency {
    fn default() -> PayPeriodDetailsPayFrequency {
        Self::PayFrequencyUnknown
    }
}




