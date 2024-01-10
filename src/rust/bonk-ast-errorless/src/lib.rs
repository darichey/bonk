mod convert;
mod ser;

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
    pub date: Date,
    pub description: String,
    pub postings: Vec<Posting>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

impl Date {
    pub fn new(year: u32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    pub fn parse(date: &str) -> Option<Self> {
        if let &[year, month, day] = &date.split('-').collect::<Vec<_>>()[..] {
            Some(Self {
                year: year.parse().ok()?,
                month: month.parse().ok()?,
                day: day.parse().ok()?,
            })
        } else {
            None
        }
    }
}

impl ToString for Date {
    fn to_string(&self) -> String {
        format!("{}-{:0>2}-{:0>2}", self.year, self.month, self.day)
    }
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
