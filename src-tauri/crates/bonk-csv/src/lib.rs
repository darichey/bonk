use std::{error::Error, io};

use bonk_ast_errorless::{Account, Amount, Ledger, Posting, Transaction};
use chrono::NaiveDate;
use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CsvTransaction {
    date: String,
    description: String,
    amount: f64,
}

pub fn do_import<D: io::Read>(
    account: &str,
    reader: &mut Reader<D>,
) -> Result<Ledger, Box<dyn Error>> {
    let account = Account::parse(account);

    let transactions = reader
        .deserialize()
        .map(|result| {
            let CsvTransaction {
                date,
                description,
                amount,
            } = result?;

            Ok(Transaction {
                date: NaiveDate::parse_from_str(&date, "%Y-%m-%d")?,
                description,
                postings: vec![Posting {
                    account: account.clone(),
                    amount: Amount {
                        cents: (amount * 100.0) as i32,
                    },
                }],
            })
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    Ok(Ledger { transactions })
}

#[cfg(test)]
mod tests {
    use bonk_ast_errorless::{Account, Amount, Ledger, Posting, Transaction};
    use chrono::NaiveDate;

    use crate::do_import;

    #[test]
    fn test() {
        let input = br#"date,description,amount
2023-01-01,Salary Deposit,2500.00
2023-01-02,Grocery Shopping,-120.50"#;
        let mut reader = csv::Reader::from_reader(&input[..]);
        let ledger = do_import("assets:my_checking", &mut reader).unwrap();
        assert_eq!(
            ledger,
            Ledger {
                transactions: vec![
                    Transaction {
                        date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                        description: "Salary Deposit".to_string(),
                        postings: vec![Posting {
                            account: Account::parse("assets:my_checking"),
                            amount: Amount { cents: 250000 }
                        }]
                    },
                    Transaction {
                        date: NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(),
                        description: "Grocery Shopping".to_string(),
                        postings: vec![Posting {
                            account: Account::parse("assets:my_checking"),
                            amount: Amount { cents: -12050 }
                        }]
                    }
                ]
            }
        )
    }
}
