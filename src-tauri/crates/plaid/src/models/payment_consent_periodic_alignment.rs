/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentConsentPeriodicAlignment : Where the payment consent period should start.  `CALENDAR`: line up with a calendar.  `CONSENT`: on the date of consent creation.

/// Where the payment consent period should start.  `CALENDAR`: line up with a calendar.  `CONSENT`: on the date of consent creation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentConsentPeriodicAlignment {
    #[serde(rename = "CALENDAR")]
    Calendar,
    #[serde(rename = "CONSENT")]
    Consent,

}

impl ToString for PaymentConsentPeriodicAlignment {
    fn to_string(&self) -> String {
        match self {
            Self::Calendar => String::from("CALENDAR"),
            Self::Consent => String::from("CONSENT"),
        }
    }
}

impl Default for PaymentConsentPeriodicAlignment {
    fn default() -> PaymentConsentPeriodicAlignment {
        Self::Calendar
    }
}



