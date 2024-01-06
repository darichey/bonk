use chrono::NaiveDate;

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
