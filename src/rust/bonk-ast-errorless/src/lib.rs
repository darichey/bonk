use bonk_parse::ast::Source;

mod ser;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ledger {
    pub imports: Vec<Import>,
    pub declare_accounts: Vec<DeclareAccount>,
    pub transactions: Vec<Transaction>,
    pub source: Option<Source>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclareAccount {
    pub account: Account,
    pub source: Option<Source>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Import {
    pub path: String, // TODO: I guess really this should be a Path node so it can hold the nested Source
    pub source: Option<Source>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    pub date: Date,
    pub description: String,
    pub postings: Vec<Posting>,
    pub source: Option<Source>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub source: Option<Source>,
}

impl Date {
    pub fn new(year: u32, month: u32, day: u32) -> Self {
        Self {
            year,
            month,
            day,
            source: None,
        }
    }

    pub fn parse(date: &str, source: Option<Source>) -> Option<Self> {
        if let &[year, month, day] = &date.split('-').collect::<Vec<_>>()[..] {
            Some(Self {
                year: year.parse().ok()?,
                month: month.parse().ok()?,
                day: day.parse().ok()?,
                source,
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
    pub source: Option<Source>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Account {
    pub path: Vec<String>,
    pub source: Option<Source>,
}

impl Account {
    /// Creates an Account from a string like "assets:foo:my_checking"
    pub fn parse(account: &str, source: Option<Source>) -> Self {
        Self {
            path: account.split(':').map(|s| s.to_string()).collect(),
            source,
        }
    }

    pub fn path_string(&self) -> String {
        self.path.join(":")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Amount {
    pub cents: i32,
    pub source: Option<Source>,
}

impl Amount {
    pub fn from_dollars(dollar_amount: f64, source: Option<Source>) -> Self {
        Self {
            cents: (dollar_amount * 100.0) as i32,
            source,
        }
    }
}
