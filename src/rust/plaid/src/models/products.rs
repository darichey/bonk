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
    #[serde(rename = "balance_plus")]
    BalancePlus,
    #[serde(rename = "beacon")]
    Beacon,
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
    #[serde(rename = "processor_payments")]
    ProcessorPayments,
    #[serde(rename = "processor_identity")]
    ProcessorIdentity,
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "cra_base_report")]
    CraBaseReport,
    #[serde(rename = "cra_income_insights")]
    CraIncomeInsights,
    #[serde(rename = "cra_partner_insights")]
    CraPartnerInsights,
    #[serde(rename = "cra_network_insights")]
    CraNetworkInsights,
    #[serde(rename = "cra_cashflow_insights")]
    CraCashflowInsights,
    #[serde(rename = "layer")]
    Layer,
    #[serde(rename = "pay_by_bank")]
    PayByBank,

}

impl std::fmt::Display for Products {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Assets => write!(f, "assets"),
            Self::Auth => write!(f, "auth"),
            Self::Balance => write!(f, "balance"),
            Self::BalancePlus => write!(f, "balance_plus"),
            Self::Beacon => write!(f, "beacon"),
            Self::Identity => write!(f, "identity"),
            Self::IdentityMatch => write!(f, "identity_match"),
            Self::Investments => write!(f, "investments"),
            Self::InvestmentsAuth => write!(f, "investments_auth"),
            Self::Liabilities => write!(f, "liabilities"),
            Self::PaymentInitiation => write!(f, "payment_initiation"),
            Self::IdentityVerification => write!(f, "identity_verification"),
            Self::Transactions => write!(f, "transactions"),
            Self::CreditDetails => write!(f, "credit_details"),
            Self::Income => write!(f, "income"),
            Self::IncomeVerification => write!(f, "income_verification"),
            Self::StandingOrders => write!(f, "standing_orders"),
            Self::Transfer => write!(f, "transfer"),
            Self::Employment => write!(f, "employment"),
            Self::RecurringTransactions => write!(f, "recurring_transactions"),
            Self::Signal => write!(f, "signal"),
            Self::Statements => write!(f, "statements"),
            Self::ProcessorPayments => write!(f, "processor_payments"),
            Self::ProcessorIdentity => write!(f, "processor_identity"),
            Self::Profile => write!(f, "profile"),
            Self::CraBaseReport => write!(f, "cra_base_report"),
            Self::CraIncomeInsights => write!(f, "cra_income_insights"),
            Self::CraPartnerInsights => write!(f, "cra_partner_insights"),
            Self::CraNetworkInsights => write!(f, "cra_network_insights"),
            Self::CraCashflowInsights => write!(f, "cra_cashflow_insights"),
            Self::Layer => write!(f, "layer"),
            Self::PayByBank => write!(f, "pay_by_bank"),
        }
    }
}

impl Default for Products {
    fn default() -> Products {
        Self::Assets
    }
}

