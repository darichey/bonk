/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LoanAccountSubtype : Valid account subtypes for loan accounts. For a list containing descriptions of each subtype, see [Account schemas](https://plaid.com/docs/api/accounts/#StandaloneAccountType-loan).

/// Valid account subtypes for loan accounts. For a list containing descriptions of each subtype, see [Account schemas](https://plaid.com/docs/api/accounts/#StandaloneAccountType-loan).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LoanAccountSubtype {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "commercial")]
    Commercial,
    #[serde(rename = "construction")]
    Construction,
    #[serde(rename = "consumer")]
    Consumer,
    #[serde(rename = "home equity")]
    HomeEquity,
    #[serde(rename = "loan")]
    Loan,
    #[serde(rename = "mortgage")]
    Mortgage,
    #[serde(rename = "line of credit")]
    LineOfCredit,
    #[serde(rename = "student")]
    Student,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "all")]
    All,

}

impl ToString for LoanAccountSubtype {
    fn to_string(&self) -> String {
        match self {
            Self::Auto => String::from("auto"),
            Self::Business => String::from("business"),
            Self::Commercial => String::from("commercial"),
            Self::Construction => String::from("construction"),
            Self::Consumer => String::from("consumer"),
            Self::HomeEquity => String::from("home equity"),
            Self::Loan => String::from("loan"),
            Self::Mortgage => String::from("mortgage"),
            Self::LineOfCredit => String::from("line of credit"),
            Self::Student => String::from("student"),
            Self::Other => String::from("other"),
            Self::All => String::from("all"),
        }
    }
}

impl Default for LoanAccountSubtype {
    fn default() -> LoanAccountSubtype {
        Self::Auto
    }
}




