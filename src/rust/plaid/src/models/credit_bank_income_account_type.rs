/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankIncomeAccountType : The account type. This will always be `depository`.

/// The account type. This will always be `depository`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreditBankIncomeAccountType {
    #[serde(rename = "depository")]
    Depository,

}

impl ToString for CreditBankIncomeAccountType {
    fn to_string(&self) -> String {
        match self {
            Self::Depository => String::from("depository"),
        }
    }
}

impl Default for CreditBankIncomeAccountType {
    fn default() -> CreditBankIncomeAccountType {
        Self::Depository
    }
}




