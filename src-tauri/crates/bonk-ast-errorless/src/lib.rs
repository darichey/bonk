mod convert;
mod ser;

use chrono::NaiveDate;
use convert::AstHasErrors;

pub fn parse(src: &str) -> Result<Ledger, AstHasErrors> {
    let ledger = bonk_ast::parse(src);
    ledger.try_into()
}

pub struct Ledger {
    pub transactions: Vec<Transaction>,
}

pub struct Transaction {
    pub date: NaiveDate,
    pub description: String,
    pub postings: Vec<Posting>,
}

pub struct Posting {
    pub account: Account,
    pub amount: Amount,
}

pub struct Account {
    pub path: Vec<String>,
}

pub struct Amount {
    pub cents: i32,
}
