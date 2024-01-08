/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// StandaloneAccountType : The schema below describes the various `types` and corresponding `subtypes` that Plaid recognizes and reports for financial institution accounts.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StandaloneAccountType {
    /// An account type holding cash, in which funds are deposited. Supported products for `depository` accounts are: Auth (`checking` and `savings` types only), Balance, Transactions, Identity, Payment Initiation, Assets, and Investments (`cash management` type only).
    #[serde(rename = "depository")]
    pub depository: String,
    /// A credit card type account. Supported products for `credit` accounts are: Balance, Transactions, Identity, and Liabilities.
    #[serde(rename = "credit")]
    pub credit: String,
    /// A loan type account. Supported products for `loan` accounts are: Balance, Liabilities, and Transactions.
    #[serde(rename = "loan")]
    pub loan: String,
    /// An investment account. Supported products for `investment` accounts are: Balance and Investments. In API versions 2018-05-22 and earlier, this type is called `brokerage`.
    #[serde(rename = "investment")]
    pub investment: String,
    /// Other or unknown account type. Supported products for `other` accounts are: Balance, Transactions, Identity, and Assets.
    #[serde(rename = "other")]
    pub other: String,
}

impl StandaloneAccountType {
    /// The schema below describes the various `types` and corresponding `subtypes` that Plaid recognizes and reports for financial institution accounts.
    pub fn new(depository: String, credit: String, loan: String, investment: String, other: String) -> StandaloneAccountType {
        StandaloneAccountType {
            depository,
            credit,
            loan,
            investment,
            other,
        }
    }
}


