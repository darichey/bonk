/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.482.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Products : A list of products that an institution can support. All Items must be initialized with at least one product. The Balance product is always available and does not need to be specified during initialization.

/// A list of products that an institution can support. All Items must be initialized with at least one product. The Balance product is always available and does not need to be specified during initialization.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Products {
    #[serde(rename = "assets")]
    Assets,
    #[serde(rename = "auth")]
    Auth,
    #[serde(rename = "balance")]
    Balance,
    #[serde(rename = "identity")]
    Identity,
    #[serde(rename = "identity_match")]
    IdentityMatch,
    #[serde(rename = "investments")]
    Investments,
    #[serde(rename = "investments_auth")]
    InvestmentsAuth,
    #[serde(rename = "liabilities")]
    Liabilities,
    #[serde(rename = "payment_initiation")]
    PaymentInitiation,
    #[serde(rename = "identity_verification")]
    IdentityVerification,
    #[serde(rename = "transactions")]
    Transactions,
    #[serde(rename = "credit_details")]
    CreditDetails,
    #[serde(rename = "income")]
    Income,
    #[serde(rename = "income_verification")]
    IncomeVerification,
    #[serde(rename = "deposit_switch")]
    DepositSwitch,
    #[serde(rename = "standing_orders")]
    StandingOrders,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "employment")]
    Employment,
    #[serde(rename = "recurring_transactions")]
    RecurringTransactions,
    #[serde(rename = "signal")]
    Signal,
    #[serde(rename = "statements")]
    Statements,

}

impl ToString for Products {
    fn to_string(&self) -> String {
        match self {
            Self::Assets => String::from("assets"),
            Self::Auth => String::from("auth"),
            Self::Balance => String::from("balance"),
            Self::Identity => String::from("identity"),
            Self::IdentityMatch => String::from("identity_match"),
            Self::Investments => String::from("investments"),
            Self::InvestmentsAuth => String::from("investments_auth"),
            Self::Liabilities => String::from("liabilities"),
            Self::PaymentInitiation => String::from("payment_initiation"),
            Self::IdentityVerification => String::from("identity_verification"),
            Self::Transactions => String::from("transactions"),
            Self::CreditDetails => String::from("credit_details"),
            Self::Income => String::from("income"),
            Self::IncomeVerification => String::from("income_verification"),
            Self::DepositSwitch => String::from("deposit_switch"),
            Self::StandingOrders => String::from("standing_orders"),
            Self::Transfer => String::from("transfer"),
            Self::Employment => String::from("employment"),
            Self::RecurringTransactions => String::from("recurring_transactions"),
            Self::Signal => String::from("signal"),
            Self::Statements => String::from("statements"),
        }
    }
}

impl Default for Products {
    fn default() -> Products {
        Self::Assets
    }
}




