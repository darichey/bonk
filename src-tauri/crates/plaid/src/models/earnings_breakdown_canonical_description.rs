/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EarningsBreakdownCanonicalDescription : Commonly used term to describe the earning line item.

/// Commonly used term to describe the earning line item.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EarningsBreakdownCanonicalDescription {
    #[serde(rename = "BONUS")]
    Bonus,
    #[serde(rename = "COMMISSION")]
    Commission,
    #[serde(rename = "OVERTIME")]
    Overtime,
    #[serde(rename = "PAID TIME OFF")]
    PaidTimeOff,
    #[serde(rename = "REGULAR PAY")]
    RegularPay,
    #[serde(rename = "VACATION")]
    Vacation,
    #[serde(rename = "BASIC ALLOWANCE HOUSING")]
    BasicAllowanceHousing,
    #[serde(rename = "BASIC ALLOWANCE SUBSISTENCE")]
    BasicAllowanceSubsistence,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "null")]
    Null,

}

impl ToString for EarningsBreakdownCanonicalDescription {
    fn to_string(&self) -> String {
        match self {
            Self::Bonus => String::from("BONUS"),
            Self::Commission => String::from("COMMISSION"),
            Self::Overtime => String::from("OVERTIME"),
            Self::PaidTimeOff => String::from("PAID TIME OFF"),
            Self::RegularPay => String::from("REGULAR PAY"),
            Self::Vacation => String::from("VACATION"),
            Self::BasicAllowanceHousing => String::from("BASIC ALLOWANCE HOUSING"),
            Self::BasicAllowanceSubsistence => String::from("BASIC ALLOWANCE SUBSISTENCE"),
            Self::Other => String::from("OTHER"),
            Self::Null => String::from("null"),
        }
    }
}

impl Default for EarningsBreakdownCanonicalDescription {
    fn default() -> EarningsBreakdownCanonicalDescription {
        Self::Bonus
    }
}




