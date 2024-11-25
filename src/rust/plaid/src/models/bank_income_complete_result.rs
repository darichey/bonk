/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BankIncomeCompleteResult : The result of the bank income report generation  `SUCCESS`: The bank income report was successfully generated and can be retrieved via `/credit/bank_income/get`.  `FAILURE`: The bank income report failed to be generated

/// The result of the bank income report generation  `SUCCESS`: The bank income report was successfully generated and can be retrieved via `/credit/bank_income/get`.  `FAILURE`: The bank income report failed to be generated
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BankIncomeCompleteResult {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,

}

impl ToString for BankIncomeCompleteResult {
    fn to_string(&self) -> String {
        match self {
            Self::Success => String::from("SUCCESS"),
            Self::Failure => String::from("FAILURE"),
        }
    }
}

impl Default for BankIncomeCompleteResult {
    fn default() -> BankIncomeCompleteResult {
        Self::Success
    }
}




