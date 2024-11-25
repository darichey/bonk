/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InvestmentTransactionSubtype : For descriptions of possible transaction types and subtypes, see the [Investment transaction types schema](https://plaid.com/docs/api/accounts/#investment-transaction-types-schema).

/// For descriptions of possible transaction types and subtypes, see the [Investment transaction types schema](https://plaid.com/docs/api/accounts/#investment-transaction-types-schema).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InvestmentTransactionSubtype {
    #[serde(rename = "account fee")]
    AccountFee,
    #[serde(rename = "adjustment")]
    Adjustment,
    #[serde(rename = "assignment")]
    Assignment,
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "buy to cover")]
    BuyToCover,
    #[serde(rename = "contribution")]
    Contribution,
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "distribution")]
    Distribution,
    #[serde(rename = "dividend")]
    Dividend,
    #[serde(rename = "dividend reinvestment")]
    DividendReinvestment,
    #[serde(rename = "exercise")]
    Exercise,
    #[serde(rename = "expire")]
    Expire,
    #[serde(rename = "fund fee")]
    FundFee,
    #[serde(rename = "interest")]
    Interest,
    #[serde(rename = "interest receivable")]
    InterestReceivable,
    #[serde(rename = "interest reinvestment")]
    InterestReinvestment,
    #[serde(rename = "legal fee")]
    LegalFee,
    #[serde(rename = "loan payment")]
    LoanPayment,
    #[serde(rename = "long-term capital gain")]
    LongTermCapitalGain,
    #[serde(rename = "long-term capital gain reinvestment")]
    LongTermCapitalGainReinvestment,
    #[serde(rename = "management fee")]
    ManagementFee,
    #[serde(rename = "margin expense")]
    MarginExpense,
    #[serde(rename = "merger")]
    Merger,
    #[serde(rename = "miscellaneous fee")]
    MiscellaneousFee,
    #[serde(rename = "non-qualified dividend")]
    NonQualifiedDividend,
    #[serde(rename = "non-resident tax")]
    NonResidentTax,
    #[serde(rename = "pending credit")]
    PendingCredit,
    #[serde(rename = "pending debit")]
    PendingDebit,
    #[serde(rename = "qualified dividend")]
    QualifiedDividend,
    #[serde(rename = "rebalance")]
    Rebalance,
    #[serde(rename = "return of principal")]
    ReturnOfPrincipal,
    #[serde(rename = "request")]
    Request,
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "sell short")]
    SellShort,
    #[serde(rename = "send")]
    Send,
    #[serde(rename = "short-term capital gain")]
    ShortTermCapitalGain,
    #[serde(rename = "short-term capital gain reinvestment")]
    ShortTermCapitalGainReinvestment,
    #[serde(rename = "spin off")]
    SpinOff,
    #[serde(rename = "split")]
    Split,
    #[serde(rename = "stock distribution")]
    StockDistribution,
    #[serde(rename = "tax")]
    Tax,
    #[serde(rename = "tax withheld")]
    TaxWithheld,
    #[serde(rename = "trade")]
    Trade,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "transfer fee")]
    TransferFee,
    #[serde(rename = "trust fee")]
    TrustFee,
    #[serde(rename = "unqualified gain")]
    UnqualifiedGain,
    #[serde(rename = "withdrawal")]
    Withdrawal,

}

impl ToString for InvestmentTransactionSubtype {
    fn to_string(&self) -> String {
        match self {
            Self::AccountFee => String::from("account fee"),
            Self::Adjustment => String::from("adjustment"),
            Self::Assignment => String::from("assignment"),
            Self::Buy => String::from("buy"),
            Self::BuyToCover => String::from("buy to cover"),
            Self::Contribution => String::from("contribution"),
            Self::Deposit => String::from("deposit"),
            Self::Distribution => String::from("distribution"),
            Self::Dividend => String::from("dividend"),
            Self::DividendReinvestment => String::from("dividend reinvestment"),
            Self::Exercise => String::from("exercise"),
            Self::Expire => String::from("expire"),
            Self::FundFee => String::from("fund fee"),
            Self::Interest => String::from("interest"),
            Self::InterestReceivable => String::from("interest receivable"),
            Self::InterestReinvestment => String::from("interest reinvestment"),
            Self::LegalFee => String::from("legal fee"),
            Self::LoanPayment => String::from("loan payment"),
            Self::LongTermCapitalGain => String::from("long-term capital gain"),
            Self::LongTermCapitalGainReinvestment => String::from("long-term capital gain reinvestment"),
            Self::ManagementFee => String::from("management fee"),
            Self::MarginExpense => String::from("margin expense"),
            Self::Merger => String::from("merger"),
            Self::MiscellaneousFee => String::from("miscellaneous fee"),
            Self::NonQualifiedDividend => String::from("non-qualified dividend"),
            Self::NonResidentTax => String::from("non-resident tax"),
            Self::PendingCredit => String::from("pending credit"),
            Self::PendingDebit => String::from("pending debit"),
            Self::QualifiedDividend => String::from("qualified dividend"),
            Self::Rebalance => String::from("rebalance"),
            Self::ReturnOfPrincipal => String::from("return of principal"),
            Self::Request => String::from("request"),
            Self::Sell => String::from("sell"),
            Self::SellShort => String::from("sell short"),
            Self::Send => String::from("send"),
            Self::ShortTermCapitalGain => String::from("short-term capital gain"),
            Self::ShortTermCapitalGainReinvestment => String::from("short-term capital gain reinvestment"),
            Self::SpinOff => String::from("spin off"),
            Self::Split => String::from("split"),
            Self::StockDistribution => String::from("stock distribution"),
            Self::Tax => String::from("tax"),
            Self::TaxWithheld => String::from("tax withheld"),
            Self::Trade => String::from("trade"),
            Self::Transfer => String::from("transfer"),
            Self::TransferFee => String::from("transfer fee"),
            Self::TrustFee => String::from("trust fee"),
            Self::UnqualifiedGain => String::from("unqualified gain"),
            Self::Withdrawal => String::from("withdrawal"),
        }
    }
}

impl Default for InvestmentTransactionSubtype {
    fn default() -> InvestmentTransactionSubtype {
        Self::AccountFee
    }
}




