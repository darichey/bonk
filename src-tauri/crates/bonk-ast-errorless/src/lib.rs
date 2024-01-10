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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Amount {
    pub cents: i32,
}
