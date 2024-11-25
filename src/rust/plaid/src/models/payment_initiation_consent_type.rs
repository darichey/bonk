/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationConsentType : Payment consent type. Defines possible use case for payments made with the given consent.  `SWEEPING`: Allows moving money between accounts owned by the same user.  `COMMERCIAL`: Allows initiating payments from the user's account to third parties.

/// Payment consent type. Defines possible use case for payments made with the given consent.  `SWEEPING`: Allows moving money between accounts owned by the same user.  `COMMERCIAL`: Allows initiating payments from the user's account to third parties.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentInitiationConsentType {
    #[serde(rename = "SWEEPING")]
    Sweeping,
    #[serde(rename = "COMMERCIAL")]
    Commercial,

}

impl ToString for PaymentInitiationConsentType {
    fn to_string(&self) -> String {
        match self {
            Self::Sweeping => String::from("SWEEPING"),
            Self::Commercial => String::from("COMMERCIAL"),
        }
    }
}

impl Default for PaymentInitiationConsentType {
    fn default() -> PaymentInitiationConsentType {
        Self::Sweeping
    }
}




