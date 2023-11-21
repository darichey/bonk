/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankIncomeErrorType : A broad categorization of the error. Safe for programmatic use.

/// A broad categorization of the error. Safe for programmatic use.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreditBankIncomeErrorType {
    #[serde(rename = "INTERNAL_SERVER_ERROR")]
    InternalServerError,
    #[serde(rename = "INSUFFICIENT_CREDENTIALS")]
    InsufficientCredentials,
    #[serde(rename = "ITEM_LOCKED")]
    ItemLocked,
    #[serde(rename = "USER_SETUP_REQUIRED")]
    UserSetupRequired,
    #[serde(rename = "COUNTRY_NOT_SUPPORTED")]
    CountryNotSupported,
    #[serde(rename = "INSTITUTION_DOWN")]
    InstitutionDown,
    #[serde(rename = "INSTITUTION_NO_LONGER_SUPPORTED")]
    InstitutionNoLongerSupported,
    #[serde(rename = "INSTITUTION_NOT_RESPONDING")]
    InstitutionNotResponding,
    #[serde(rename = "INVALID_CREDENTIALS")]
    InvalidCredentials,
    #[serde(rename = "INVALID_MFA")]
    InvalidMfa,
    #[serde(rename = "INVALID_SEND_METHOD")]
    InvalidSendMethod,
    #[serde(rename = "ITEM_LOGIN_REQUIRED")]
    ItemLoginRequired,
    #[serde(rename = "MFA_NOT_SUPPORTED")]
    MfaNotSupported,
    #[serde(rename = "NO_ACCOUNTS")]
    NoAccounts,
    #[serde(rename = "ITEM_NOT_SUPPORTED")]
    ItemNotSupported,
    #[serde(rename = "ACCESS_NOT_GRANTED")]
    AccessNotGranted,

}

impl ToString for CreditBankIncomeErrorType {
    fn to_string(&self) -> String {
        match self {
            Self::InternalServerError => String::from("INTERNAL_SERVER_ERROR"),
            Self::InsufficientCredentials => String::from("INSUFFICIENT_CREDENTIALS"),
            Self::ItemLocked => String::from("ITEM_LOCKED"),
            Self::UserSetupRequired => String::from("USER_SETUP_REQUIRED"),
            Self::CountryNotSupported => String::from("COUNTRY_NOT_SUPPORTED"),
            Self::InstitutionDown => String::from("INSTITUTION_DOWN"),
            Self::InstitutionNoLongerSupported => String::from("INSTITUTION_NO_LONGER_SUPPORTED"),
            Self::InstitutionNotResponding => String::from("INSTITUTION_NOT_RESPONDING"),
            Self::InvalidCredentials => String::from("INVALID_CREDENTIALS"),
            Self::InvalidMfa => String::from("INVALID_MFA"),
            Self::InvalidSendMethod => String::from("INVALID_SEND_METHOD"),
            Self::ItemLoginRequired => String::from("ITEM_LOGIN_REQUIRED"),
            Self::MfaNotSupported => String::from("MFA_NOT_SUPPORTED"),
            Self::NoAccounts => String::from("NO_ACCOUNTS"),
            Self::ItemNotSupported => String::from("ITEM_NOT_SUPPORTED"),
            Self::AccessNotGranted => String::from("ACCESS_NOT_GRANTED"),
        }
    }
}

impl Default for CreditBankIncomeErrorType {
    fn default() -> CreditBankIncomeErrorType {
        Self::InternalServerError
    }
}



