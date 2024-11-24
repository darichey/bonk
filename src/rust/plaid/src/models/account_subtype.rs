/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.586.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AccountSubtype : See the [Account type schema](https://plaid.com/docs/api/accounts/#account-type-schema) for a full listing of account types and corresponding subtypes.

/// See the [Account type schema](https://plaid.com/docs/api/accounts/#account-type-schema) for a full listing of account types and corresponding subtypes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountSubtype {
    #[serde(rename = "401a")]
    Variant401a,
    #[serde(rename = "401k")]
    Variant401k,
    #[serde(rename = "403B")]
    Variant403B,
    #[serde(rename = "457b")]
    Variant457b,
    #[serde(rename = "529")]
    Variant529,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "brokerage")]
    Brokerage,
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "cash isa")]
    CashIsa,
    #[serde(rename = "cash management")]
    CashManagement,
    #[serde(rename = "cd")]
    Cd,
    #[serde(rename = "checking")]
    Checking,
    #[serde(rename = "commercial")]
    Commercial,
    #[serde(rename = "construction")]
    Construction,
    #[serde(rename = "consumer")]
    Consumer,
    #[serde(rename = "credit card")]
    CreditCard,
    #[serde(rename = "crypto exchange")]
    CryptoExchange,
    #[serde(rename = "ebt")]
    Ebt,
    #[serde(rename = "education savings account")]
    EducationSavingsAccount,
    #[serde(rename = "fixed annuity")]
    FixedAnnuity,
    #[serde(rename = "gic")]
    Gic,
    #[serde(rename = "health reimbursement arrangement")]
    HealthReimbursementArrangement,
    #[serde(rename = "home equity")]
    HomeEquity,
    #[serde(rename = "hsa")]
    Hsa,
    #[serde(rename = "isa")]
    Isa,
    #[serde(rename = "ira")]
    Ira,
    #[serde(rename = "keogh")]
    Keogh,
    #[serde(rename = "lif")]
    Lif,
    #[serde(rename = "life insurance")]
    LifeInsurance,
    #[serde(rename = "line of credit")]
    LineOfCredit,
    #[serde(rename = "lira")]
    Lira,
    #[serde(rename = "loan")]
    Loan,
    #[serde(rename = "lrif")]
    Lrif,
    #[serde(rename = "lrsp")]
    Lrsp,
    #[serde(rename = "money market")]
    MoneyMarket,
    #[serde(rename = "mortgage")]
    Mortgage,
    #[serde(rename = "mutual fund")]
    MutualFund,
    #[serde(rename = "non-custodial wallet")]
    NonCustodialWallet,
    #[serde(rename = "non-taxable brokerage account")]
    NonTaxableBrokerageAccount,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "other insurance")]
    OtherInsurance,
    #[serde(rename = "other annuity")]
    OtherAnnuity,
    #[serde(rename = "overdraft")]
    Overdraft,
    #[serde(rename = "paypal")]
    Paypal,
    #[serde(rename = "payroll")]
    Payroll,
    #[serde(rename = "pension")]
    Pension,
    #[serde(rename = "prepaid")]
    Prepaid,
    #[serde(rename = "prif")]
    Prif,
    #[serde(rename = "profit sharing plan")]
    ProfitSharingPlan,
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
    #[serde(rename = "savings")]
    Savings,
    #[serde(rename = "sep ira")]
    SepIra,
    #[serde(rename = "simple ira")]
    SimpleIra,
    #[serde(rename = "sipp")]
    Sipp,
    #[serde(rename = "stock plan")]
    StockPlan,
    #[serde(rename = "student")]
    Student,
    #[serde(rename = "thrift savings plan")]
    ThriftSavingsPlan,
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

}

impl ToString for AccountSubtype {
    fn to_string(&self) -> String {
        match self {
            Self::Variant401a => String::from("401a"),
            Self::Variant401k => String::from("401k"),
            Self::Variant403B => String::from("403B"),
            Self::Variant457b => String::from("457b"),
            Self::Variant529 => String::from("529"),
            Self::Auto => String::from("auto"),
            Self::Brokerage => String::from("brokerage"),
            Self::Business => String::from("business"),
            Self::CashIsa => String::from("cash isa"),
            Self::CashManagement => String::from("cash management"),
            Self::Cd => String::from("cd"),
            Self::Checking => String::from("checking"),
            Self::Commercial => String::from("commercial"),
            Self::Construction => String::from("construction"),
            Self::Consumer => String::from("consumer"),
            Self::CreditCard => String::from("credit card"),
            Self::CryptoExchange => String::from("crypto exchange"),
            Self::Ebt => String::from("ebt"),
            Self::EducationSavingsAccount => String::from("education savings account"),
            Self::FixedAnnuity => String::from("fixed annuity"),
            Self::Gic => String::from("gic"),
            Self::HealthReimbursementArrangement => String::from("health reimbursement arrangement"),
            Self::HomeEquity => String::from("home equity"),
            Self::Hsa => String::from("hsa"),
            Self::Isa => String::from("isa"),
            Self::Ira => String::from("ira"),
            Self::Keogh => String::from("keogh"),
            Self::Lif => String::from("lif"),
            Self::LifeInsurance => String::from("life insurance"),
            Self::LineOfCredit => String::from("line of credit"),
            Self::Lira => String::from("lira"),
            Self::Loan => String::from("loan"),
            Self::Lrif => String::from("lrif"),
            Self::Lrsp => String::from("lrsp"),
            Self::MoneyMarket => String::from("money market"),
            Self::Mortgage => String::from("mortgage"),
            Self::MutualFund => String::from("mutual fund"),
            Self::NonCustodialWallet => String::from("non-custodial wallet"),
            Self::NonTaxableBrokerageAccount => String::from("non-taxable brokerage account"),
            Self::Other => String::from("other"),
            Self::OtherInsurance => String::from("other insurance"),
            Self::OtherAnnuity => String::from("other annuity"),
            Self::Overdraft => String::from("overdraft"),
            Self::Paypal => String::from("paypal"),
            Self::Payroll => String::from("payroll"),
            Self::Pension => String::from("pension"),
            Self::Prepaid => String::from("prepaid"),
            Self::Prif => String::from("prif"),
            Self::ProfitSharingPlan => String::from("profit sharing plan"),
            Self::Rdsp => String::from("rdsp"),
            Self::Resp => String::from("resp"),
            Self::Retirement => String::from("retirement"),
            Self::Rlif => String::from("rlif"),
            Self::Roth => String::from("roth"),
            Self::Roth401k => String::from("roth 401k"),
            Self::Rrif => String::from("rrif"),
            Self::Rrsp => String::from("rrsp"),
            Self::Sarsep => String::from("sarsep"),
            Self::Savings => String::from("savings"),
            Self::SepIra => String::from("sep ira"),
            Self::SimpleIra => String::from("simple ira"),
            Self::Sipp => String::from("sipp"),
            Self::StockPlan => String::from("stock plan"),
            Self::Student => String::from("student"),
            Self::ThriftSavingsPlan => String::from("thrift savings plan"),
            Self::Tfsa => String::from("tfsa"),
            Self::Trust => String::from("trust"),
            Self::Ugma => String::from("ugma"),
            Self::Utma => String::from("utma"),
            Self::VariableAnnuity => String::from("variable annuity"),
        }
    }
}

impl Default for AccountSubtype {
    fn default() -> AccountSubtype {
        Self::Variant401a
    }
}




