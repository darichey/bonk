use chrono::NaiveDate;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, digit1, line_ending, newline, not_line_ending},
    combinator::{map, map_opt, map_res, opt},
    multi::{many1, many_till, separated_list1},
    number::complete::float,
    sequence::preceded,
    IResult,
};

use crate::types::{Account, Amount, Ledger, Posting, Transaction};

pub fn ledger(input: &str) -> IResult<&str, Ledger> {
    map(
        separated_list1(many1(newline), transaction),
        |transactions| Ledger { transactions },
    )(input)
}

fn transaction(input: &str) -> IResult<&str, Transaction> {
    let (input, date) = date(input)?;
    let (input, description) = map(
        preceded(spaces, many_till(not_line_ending, line_ending)),
        |(ss, _)| ss.join(""),
    )(input)?;
    let (input, postings) = separated_list1(newline, preceded(spaces, posting))(input)?;
    let transaction = Transaction {
        date,
        description,
        postings,
    };
    Ok((input, transaction))
}

fn posting(input: &str) -> IResult<&str, Posting> {
    let (input, account) = account(input)?;
    let (input, amount) = opt(preceded(spaces, amount))(input)?;
    let posting = Posting { account, amount };
    Ok((input, posting))
}

fn date(input: &str) -> IResult<&str, NaiveDate> {
    map_opt(
        |input| {
            let (input, year) = map_res(digit1, str::parse::<i32>)(input)?;
            let (input, _) = char('-')(input)?;
            let (input, month) = map_res(digit1, str::parse::<u32>)(input)?;
            let (input, _) = char('-')(input)?;
            let (input, day) = map_res(digit1, str::parse::<u32>)(input)?;
            Ok((input, (year, month, day)))
        },
        |(y, m, d)| NaiveDate::from_ymd_opt(y, m, d),
    )(input)
}

fn account(input: &str) -> IResult<&str, Account> {
    let (input, path) = separated_list1(char(':'), account_component)(input)?;
    let account = Account { path };
    Ok((input, account))
}

fn account_component(input: &str) -> IResult<&str, String> {
    map(many1(alt((alphanumeric1, tag("_")))), |ss| ss.join(""))(input)
}

fn amount(input: &str) -> IResult<&str, Amount> {
    let (input, amount) = float(input)?;
    let amount = Amount {
        cents: (amount * 100.0) as i32,
    };
    Ok((input, amount))
}

fn spaces(input: &str) -> IResult<&str, ()> {
    let (input, _) = many1(tag(" "))(input)?;
    Ok((input, ()))
}

mod tests {
    use super::*;

    #[test]
    fn parses_ledger() {
        assert_eq!(
            ledger(
                "2023-01-01 Mcdonald's
  expenses:fast_food         10.91
  liabilities:my_credit_card

2023-01-02 Paying credit card
  liabilities:my_credit_card    10.91
  assets:my_checking"
            ),
            Ok((
                "",
                Ledger {
                    transactions: vec![
                        Transaction {
                            date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                            description: "Mcdonald's".to_string(),
                            postings: vec![
                                Posting {
                                    account: Account {
                                        path: vec!["expenses".to_string(), "fast_food".to_string()]
                                    },
                                    amount: Some(Amount { cents: 1091 })
                                },
                                Posting {
                                    account: Account {
                                        path: vec![
                                            "liabilities".to_string(),
                                            "my_credit_card".to_string()
                                        ]
                                    },
                                    amount: None,
                                }
                            ]
                        },
                        Transaction {
                            date: NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(),
                            description: "Paying credit card".to_string(),
                            postings: vec![
                                Posting {
                                    account: Account {
                                        path: vec![
                                            "liabilities".to_string(),
                                            "my_credit_card".to_string()
                                        ]
                                    },
                                    amount: Some(Amount { cents: 1091 })
                                },
                                Posting {
                                    account: Account {
                                        path: vec!["assets".to_string(), "my_checking".to_string()]
                                    },
                                    amount: None,
                                }
                            ]
                        }
                    ]
                }
            ))
        )
    }

    #[test]
    fn parses_transactions() {
        assert_eq!(
            transaction(
                "2023-01-01 Mcdonald's
  expenses:fast_food         10.91
  liabilities:my_credit_card"
            ),
            Ok((
                "",
                Transaction {
                    date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                    description: "Mcdonald's".to_string(),
                    postings: vec![
                        Posting {
                            account: Account {
                                path: vec!["expenses".to_string(), "fast_food".to_string()]
                            },
                            amount: Some(Amount { cents: 1091 })
                        },
                        Posting {
                            account: Account {
                                path: vec!["liabilities".to_string(), "my_credit_card".to_string()]
                            },
                            amount: None,
                        }
                    ]
                }
            ))
        )
    }

    #[test]
    fn parses_postings() {
        assert_eq!(
            posting("expenses:fast_food 10.91"),
            Ok((
                "",
                Posting {
                    account: Account {
                        path: vec!["expenses".to_string(), "fast_food".to_string()]
                    },
                    amount: Some(Amount { cents: 1091 })
                }
            ))
        );

        assert_eq!(
            posting("expenses:fast_food       10.91"),
            Ok((
                "",
                Posting {
                    account: Account {
                        path: vec!["expenses".to_string(), "fast_food".to_string()]
                    },
                    amount: Some(Amount { cents: 1091 })
                }
            ))
        );

        assert_eq!(
            posting("expenses:fast_food"),
            Ok((
                "",
                Posting {
                    account: Account {
                        path: vec!["expenses".to_string(), "fast_food".to_string()]
                    },
                    amount: None,
                }
            ))
        );
    }

    #[test]
    fn parses_dates() {
        assert_eq!(
            date("2023-01-01"),
            Ok(("", NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()))
        );

        assert_eq!(
            date("2024-11-29"),
            Ok(("", NaiveDate::from_ymd_opt(2024, 11, 29).unwrap()))
        )
    }

    #[test]
    fn parses_accounts() {
        assert_eq!(
            account("x"),
            Ok((
                "",
                Account {
                    path: vec!["x".to_string()]
                }
            ))
        );

        assert_eq!(
            account("x:y:z"),
            Ok((
                "",
                Account {
                    path: vec!["x".to_string(), "y".to_string(), "z".to_string()]
                }
            ))
        );

        assert_eq!(
            account("foo_bar"),
            Ok((
                "",
                Account {
                    path: vec!["foo_bar".to_string()]
                }
            ))
        );

        assert_eq!(
            account("foo:bar_baz:qux"),
            Ok((
                "",
                Account {
                    path: vec!["foo".to_string(), "bar_baz".to_string(), "qux".to_string()]
                }
            ))
        );
    }

    #[test]
    fn parses_amounts() {
        assert_eq!(amount("3.14"), Ok(("", Amount { cents: 314 })));

        assert_eq!(amount("10.91"), Ok(("", Amount { cents: 1091 })));

        assert_eq!(amount("100"), Ok(("", Amount { cents: 10000 })));
    }
}
