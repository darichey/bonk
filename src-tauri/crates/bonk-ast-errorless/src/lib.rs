mod convert;
mod ser;

use chrono::NaiveDate;
use convert::AstHasErrors;

pub fn parse(src: &str) -> Result<Ledger, AstHasErrors> {
    let ledger = bonk_ast::parse(src);
    ledger.try_into()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ledger {
    pub transactions: Vec<Transaction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    pub date: NaiveDate,
    pub description: String,
    pub postings: Vec<Posting>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Posting {
    pub account: Account,
    pub amount: Amount,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Account {
    pub path: Vec<String>,
}

impl Account {
    /// Creates an Account from a string like "assets:foo:my_checking"
    pub fn parse(account: &str) -> Self {
        Self {
            path: account.split(':').map(|s| s.to_string()).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Amount {
    pub cents: i32,
}

impl Amount {
    pub fn from_dollars(dollar_amount: f64) -> Self {
        Self {
            cents: (dollar_amount * 100.0) as i32,
        }
    }
}
