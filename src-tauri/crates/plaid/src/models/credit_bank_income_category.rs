/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankIncomeCategory : The income category. Note that the `CASH` value has been deprecated and is used only for existing legacy implementations. It has been replaced by the new categories `CASH_DEPOSIT` (representing cash or check deposits) and `TRANSFER_FROM_APPLICATION` (representing cash transfers originating from apps, such as Zelle or Venmo).

/// The income category. Note that the `CASH` value has been deprecated and is used only for existing legacy implementations. It has been replaced by the new categories `CASH_DEPOSIT` (representing cash or check deposits) and `TRANSFER_FROM_APPLICATION` (representing cash transfers originating from apps, such as Zelle or Venmo).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreditBankIncomeCategory {
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
    #[serde(rename = "CASH_DEPOSIT")]
    CashDeposit,
    #[serde(rename = "TRANSFER_FROM_APPLICATION")]
    TransferFromApplication,
    #[serde(rename = "TAX_REFUND")]
    TaxRefund,
    #[serde(rename = "BENEFIT_OTHER")]
    BenefitOther,
    #[serde(rename = "OTHER")]
    Other,

}

impl ToString for CreditBankIncomeCategory {
    fn to_string(&self) -> String {
        match self {
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
            Self::CashDeposit => String::from("CASH_DEPOSIT"),
            Self::TransferFromApplication => String::from("TRANSFER_FROM_APPLICATION"),
            Self::TaxRefund => String::from("TAX_REFUND"),
            Self::BenefitOther => String::from("BENEFIT_OTHER"),
            Self::Other => String::from("OTHER"),
        }
    }
}

impl Default for CreditBankIncomeCategory {
    fn default() -> CreditBankIncomeCategory {
        Self::Salary
    }
}




