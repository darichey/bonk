pub mod cli;

use std::io;

use anyhow::Context;
use bonk_ast_errorless::{Account, Amount, Date, Ledger, Posting, Transaction};
use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CsvTransaction {
    date: String,
    description: String,
    amount: f64,
}

pub fn do_convert<D: io::Read>(account: &str, reader: &mut Reader<D>) -> anyhow::Result<Ledger> {
    let account = Account::parse(account, None);

    let transactions = reader
        .deserialize()
        .map(|result| {
            let CsvTransaction {
                date,
                description,
                amount,
            } = result?;

            Ok(Transaction {
                date: Date::parse(&date, None).context("Couldn't parse date")?,
                description,
                postings: vec![Posting {
                    account: account.clone(),
                    amount: Some(Amount::from_dollars(amount, None)),
                    source: None,
                }],
                source: None,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(Ledger {
        declare_accounts: vec![],
        transactions,
        source: None,
    })
}

#[cfg(test)]
mod tests {
    use crate::do_convert;

    #[test]
    fn test() {
        let input = br#"date,description,amount
2023-01-01,Salary Deposit,2500.00
2023-01-02,Grocery Shopping,-120.50"#;
        let mut reader = csv::Reader::from_reader(&input[..]);
        let ledger = do_convert("assets/my_checking", &mut reader).unwrap();

        insta::assert_debug_snapshot!(ledger, @r###"
        Ledger {
            declare_accounts: [],
            transactions: [
                Transaction {
                    date: Date {
                        year: 2023,
                        month: 1,
                        day: 1,
                        source: None,
                    },
                    description: "Salary Deposit",
                    postings: [
                        Posting {
                            account: Account {
                                path: [
                                    "assets",
                                    "my_checking",
                                ],
                                source: None,
                            },
                            amount: Some(
                                Amount {
                                    cents: 250000,
                                    source: None,
                                },
                            ),
                            source: None,
                        },
                    ],
                    source: None,
                },
                Transaction {
                    date: Date {
                        year: 2023,
                        month: 1,
                        day: 2,
                        source: None,
                    },
                    description: "Grocery Shopping",
                    postings: [
                        Posting {
                            account: Account {
                                path: [
                                    "assets",
                                    "my_checking",
                                ],
                                source: None,
                            },
                            amount: Some(
                                Amount {
                                    cents: -12050,
                                    source: None,
                                },
                            ),
                            source: None,
                        },
                    ],
                    source: None,
                },
            ],
            source: None,
        }
        "###);
    }
}
