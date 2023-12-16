use chrono::NaiveDate;

#[derive(PartialEq, Eq, Debug)]
pub struct Ledger {
    pub transactions: Vec<Transaction>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Transaction {
    pub date: NaiveDate,
    pub description: String,
    pub postings: Vec<Posting>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Posting {
    pub account: Account,
    pub amount: Option<Amount>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Account {
    pub path: Vec<String>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Amount {
    pub cents: i32,
}
