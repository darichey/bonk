/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserStatedIncomeSourceCategory : The income category for a specified income source

/// The income category for a specified income source
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserStatedIncomeSourceCategory {
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "SALARY")]
    Salary,
    #[serde(rename = "UNEMPLOYMENT")]
    Unemployment,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "GIG_ECONOMY")]
    GigEconomy,
    #[serde(rename = "RENTAL")]
    Rental,
    #[serde(rename = "CHILD_SUPPORT")]
    ChildSupport,
    #[serde(rename = "MILITARY")]
    Military,
    #[serde(rename = "RETIREMENT")]
    Retirement,
    #[serde(rename = "LONG_TERM_DISABILITY")]
    LongTermDisability,
    #[serde(rename = "BANK_INTEREST")]
    BankInterest,

}

impl ToString for UserStatedIncomeSourceCategory {
    fn to_string(&self) -> String {
        match self {
            Self::Other => String::from("OTHER"),
            Self::Salary => String::from("SALARY"),
            Self::Unemployment => String::from("UNEMPLOYMENT"),
            Self::Cash => String::from("CASH"),
            Self::GigEconomy => String::from("GIG_ECONOMY"),
            Self::Rental => String::from("RENTAL"),
            Self::ChildSupport => String::from("CHILD_SUPPORT"),
            Self::Military => String::from("MILITARY"),
            Self::Retirement => String::from("RETIREMENT"),
            Self::LongTermDisability => String::from("LONG_TERM_DISABILITY"),
            Self::BankInterest => String::from("BANK_INTEREST"),
        }
    }
}

impl Default for UserStatedIncomeSourceCategory {
    fn default() -> UserStatedIncomeSourceCategory {
        Self::Other
    }
}




