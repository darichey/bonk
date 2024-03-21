use std::collections::HashMap;

use bonk_parse::ast::Source;

mod ser;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ledger {
    pub declare_accounts: Vec<DeclareAccount>,
    pub transactions: Vec<Transaction>,
    pub source: Option<Source>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclareAccount {
    pub account: Account,
    pub metadata: Metadata,
    pub source: Option<Source>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Metadata {
    pub entries: HashMap<String, String>,
}

impl Metadata {
    pub fn new() -> Self {
        Metadata {
            entries: HashMap::new(),
        }
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new()
    }
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
    pub amount: Option<Amount>,
    pub source: Option<Source>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Account {
    pub path: Vec<String>,
    pub source: Option<Source>,
}

impl Account {
    /// Creates an Account from a string like "assets/foo/my_checking"
    pub fn parse(account: &str, source: Option<Source>) -> Self {
        Self {
            path: account.split('/').map(|s| s.to_string()).collect(),
            source,
        }
    }

    pub fn path_string(&self) -> String {
        self.path.join("/")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Amount {
    pub cents: i32,
    pub source: Option<Source>,
}

impl Amount {
    pub fn parse(amount: &str, source: Option<Source>) -> Result<Self, std::num::ParseFloatError> {
        Ok(Self::from_dollars(amount.parse::<f64>()?, source))
    }

    pub fn from_dollars(dollar_amount: f64, source: Option<Source>) -> Self {
        Self {
            cents: (dollar_amount * 100.0).round() as i32,
            source,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Amount;

    #[test]
    fn test_amount_parse() {
        assert_eq!(
            Amount::parse("3", None),
            Ok(Amount {
                cents: 300,
                source: None,
            })
        );

        assert_eq!(
            Amount::parse("3.0", None),
            Ok(Amount {
                cents: 300,
                source: None,
            })
        );

        assert_eq!(
            Amount::parse("3.00", None),
            Ok(Amount {
                cents: 300,
                source: None,
            })
        );

        assert_eq!(
            Amount::parse("03", None),
            Ok(Amount {
                cents: 300,
                source: None,
            })
        );

        assert_eq!(
            Amount::parse("31.4", None),
            Ok(Amount {
                cents: 3140,
                source: None,
            })
        );

        assert_eq!(
            Amount::parse("3.14", None),
            Ok(Amount {
                cents: 314,
                source: None,
            })
        );

        assert_eq!(
            Amount::parse("30.40", None),
            Ok(Amount {
                cents: 3040,
                source: None,
            })
        );

        assert_eq!(
            Amount::parse("2500.0", None),
            Ok(Amount {
                cents: 250000,
                source: None,
            })
        );
    }

    #[test]
    fn test_amount_from_dollars() {
        assert_eq!(
            Amount::from_dollars(1.0, None),
            Amount {
                cents: 100,
                source: None
            }
        );

        assert_eq!(
            Amount::from_dollars(100.0, None),
            Amount {
                cents: 10_000,
                source: None
            }
        );

        assert_eq!(
            Amount::from_dollars(34.05, None),
            Amount {
                cents: 3405,
                source: None
            }
        );
    }
}
