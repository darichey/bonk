use bonk_ast::SourceSpan;

mod ser;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ledger {
    pub declare_accounts: Vec<DeclareAccount>,
    pub transactions: Vec<Transaction>,
    pub source_span: Option<SourceSpan>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclareAccount {
    pub account: Account,
    pub source_span: Option<SourceSpan>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    pub date: Date,
    pub description: String,
    pub postings: Vec<Posting>,
    pub source_span: Option<SourceSpan>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub source_span: Option<SourceSpan>,
}

impl Date {
    pub fn new(year: u32, month: u32, day: u32) -> Self {
        Self {
            year,
            month,
            day,
            source_span: None,
        }
    }

    pub fn parse(date: &str, source_span: Option<SourceSpan>) -> Option<Self> {
        if let &[year, month, day] = &date.split('-').collect::<Vec<_>>()[..] {
            Some(Self {
                year: year.parse().ok()?,
                month: month.parse().ok()?,
                day: day.parse().ok()?,
                source_span,
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
    pub source_span: Option<SourceSpan>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Account {
    pub path: Vec<String>,
    pub source_span: Option<SourceSpan>,
}

impl Account {
    /// Creates an Account from a string like "assets:foo:my_checking"
    pub fn parse(account: &str, source_span: Option<SourceSpan>) -> Self {
        Self {
            path: account.split(':').map(|s| s.to_string()).collect(),
            source_span,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Amount {
    pub cents: i32,
    pub source_span: Option<SourceSpan>,
}

impl Amount {
    pub fn from_dollars(dollar_amount: f64, source_span: Option<SourceSpan>) -> Self {
        Self {
            cents: (dollar_amount * 100.0) as i32,
            source_span,
        }
    }
}
