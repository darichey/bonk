/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OverrideAccountType : `investment:` Investment account.  `credit:` Credit card  `depository:` Depository account  `loan:` Loan account  `payroll:` Payroll account  `other:` Non-specified account type  See the [Account type schema](https://plaid.com/docs/api/accounts#account-type-schema) for a full listing of account types and corresponding subtypes.

/// `investment:` Investment account.  `credit:` Credit card  `depository:` Depository account  `loan:` Loan account  `payroll:` Payroll account  `other:` Non-specified account type  See the [Account type schema](https://plaid.com/docs/api/accounts#account-type-schema) for a full listing of account types and corresponding subtypes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OverrideAccountType {
    #[serde(rename = "investment")]
    Investment,
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "depository")]
    Depository,
    #[serde(rename = "loan")]
    Loan,
    #[serde(rename = "payroll")]
    Payroll,
    #[serde(rename = "other")]
    Other,

}

impl ToString for OverrideAccountType {
    fn to_string(&self) -> String {
        match self {
            Self::Investment => String::from("investment"),
            Self::Credit => String::from("credit"),
            Self::Depository => String::from("depository"),
            Self::Loan => String::from("loan"),
            Self::Payroll => String::from("payroll"),
            Self::Other => String::from("other"),
        }
    }
}

impl Default for OverrideAccountType {
    fn default() -> OverrideAccountType {
        Self::Investment
    }
}



