/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// InvestmentAccountSubtype : Valid account subtypes for investment accounts. For a list containing descriptions of each subtype, see [Account schemas](https://plaid.com/docs/api/accounts/#StandaloneAccountType-investment).
/// Valid account subtypes for investment accounts. For a list containing descriptions of each subtype, see [Account schemas](https://plaid.com/docs/api/accounts/#StandaloneAccountType-investment).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InvestmentAccountSubtype {
    #[serde(rename = "529")]
    Variant529,
    #[serde(rename = "401a")]
    Variant401a,
    #[serde(rename = "401k")]
    Variant401k,
    #[serde(rename = "403B")]
    Variant403B,
    #[serde(rename = "457b")]
    Variant457b,
    #[serde(rename = "brokerage")]
    Brokerage,
    #[serde(rename = "cash isa")]
    CashIsa,
    #[serde(rename = "crypto exchange")]
    CryptoExchange,
    #[serde(rename = "education savings account")]
    EducationSavingsAccount,
    #[serde(rename = "fixed annuity")]
    FixedAnnuity,
    #[serde(rename = "gic")]
    Gic,
    #[serde(rename = "health reimbursement arrangement")]
    HealthReimbursementArrangement,
    #[serde(rename = "hsa")]
    Hsa,
    #[serde(rename = "ira")]
    Ira,
    #[serde(rename = "isa")]
    Isa,
    #[serde(rename = "keogh")]
    Keogh,
    #[serde(rename = "lif")]
    Lif,
    #[serde(rename = "life insurance")]
    LifeInsurance,
    #[serde(rename = "lira")]
    Lira,
    #[serde(rename = "lrif")]
    Lrif,
    #[serde(rename = "lrsp")]
    Lrsp,
    #[serde(rename = "mutual fund")]
    MutualFund,
    #[serde(rename = "non-custodial wallet")]
    NonCustodialWallet,
    #[serde(rename = "non-taxable brokerage account")]
    NonTaxableBrokerageAccount,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "other annuity")]
    OtherAnnuity,
    #[serde(rename = "other insurance")]
    OtherInsurance,
    #[serde(rename = "pension")]
    Pension,
    #[serde(rename = "prif")]
    Prif,
    #[serde(rename = "profit sharing plan")]
    ProfitSharingPlan,
    #[serde(rename = "qshr")]
    Qshr,
    #[serde(rename = "rdsp")]
    Rdsp,
    #[serde(rename = "resp")]
    Resp,
    #[serde(rename = "retirement")]
    Retirement,
    #[serde(rename = "rlif")]
    Rlif,
    #[serde(rename = "roth")]
    Roth,
    #[serde(rename = "roth 401k")]
    Roth401k,
    #[serde(rename = "rrif")]
    Rrif,
    #[serde(rename = "rrsp")]
    Rrsp,
    #[serde(rename = "sarsep")]
    Sarsep,
    #[serde(rename = "sep ira")]
    SepIra,
    #[serde(rename = "simple ira")]
    SimpleIra,
    #[serde(rename = "sipp")]
    Sipp,
    #[serde(rename = "stock plan")]
    StockPlan,
    #[serde(rename = "tfsa")]
    Tfsa,
    #[serde(rename = "trust")]
    Trust,
    #[serde(rename = "ugma")]
    Ugma,
    #[serde(rename = "utma")]
    Utma,
    #[serde(rename = "variable annuity")]
    VariableAnnuity,
    #[serde(rename = "all")]
    All,

}

impl std::fmt::Display for InvestmentAccountSubtype {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant529 => write!(f, "529"),
            Self::Variant401a => write!(f, "401a"),
            Self::Variant401k => write!(f, "401k"),
            Self::Variant403B => write!(f, "403B"),
            Self::Variant457b => write!(f, "457b"),
            Self::Brokerage => write!(f, "brokerage"),
            Self::CashIsa => write!(f, "cash isa"),
            Self::CryptoExchange => write!(f, "crypto exchange"),
            Self::EducationSavingsAccount => write!(f, "education savings account"),
            Self::FixedAnnuity => write!(f, "fixed annuity"),
            Self::Gic => write!(f, "gic"),
            Self::HealthReimbursementArrangement => write!(f, "health reimbursement arrangement"),
            Self::Hsa => write!(f, "hsa"),
            Self::Ira => write!(f, "ira"),
            Self::Isa => write!(f, "isa"),
            Self::Keogh => write!(f, "keogh"),
            Self::Lif => write!(f, "lif"),
            Self::LifeInsurance => write!(f, "life insurance"),
            Self::Lira => write!(f, "lira"),
            Self::Lrif => write!(f, "lrif"),
            Self::Lrsp => write!(f, "lrsp"),
            Self::MutualFund => write!(f, "mutual fund"),
            Self::NonCustodialWallet => write!(f, "non-custodial wallet"),
            Self::NonTaxableBrokerageAccount => write!(f, "non-taxable brokerage account"),
            Self::Other => write!(f, "other"),
            Self::OtherAnnuity => write!(f, "other annuity"),
            Self::OtherInsurance => write!(f, "other insurance"),
            Self::Pension => write!(f, "pension"),
            Self::Prif => write!(f, "prif"),
            Self::ProfitSharingPlan => write!(f, "profit sharing plan"),
            Self::Qshr => write!(f, "qshr"),
            Self::Rdsp => write!(f, "rdsp"),
            Self::Resp => write!(f, "resp"),
            Self::Retirement => write!(f, "retirement"),
            Self::Rlif => write!(f, "rlif"),
            Self::Roth => write!(f, "roth"),
            Self::Roth401k => write!(f, "roth 401k"),
            Self::Rrif => write!(f, "rrif"),
            Self::Rrsp => write!(f, "rrsp"),
            Self::Sarsep => write!(f, "sarsep"),
            Self::SepIra => write!(f, "sep ira"),
            Self::SimpleIra => write!(f, "simple ira"),
            Self::Sipp => write!(f, "sipp"),
            Self::StockPlan => write!(f, "stock plan"),
            Self::Tfsa => write!(f, "tfsa"),
            Self::Trust => write!(f, "trust"),
            Self::Ugma => write!(f, "ugma"),
            Self::Utma => write!(f, "utma"),
            Self::VariableAnnuity => write!(f, "variable annuity"),
            Self::All => write!(f, "all"),
        }
    }
}

impl Default for InvestmentAccountSubtype {
    fn default() -> InvestmentAccountSubtype {
        Self::Variant529
    }
}

